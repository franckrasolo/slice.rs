use std::fmt::Write;
use std::path::Path;

use anyhow::{self, Context, Error, Result};
use git2::{Delta, Diff, DiffDelta, DiffOptions, Repository};

use crate::{zero_oid, Config};

pub struct PrePush {
    repo: Repository,
    config: Config,
}

#[derive(Debug)]
pub struct Summary {
    pub contents: String,
}

impl PrePush {
    pub fn new(repo_path: &Path, config: Config) -> Self {
        Self { repo: Repository::open(repo_path).expect("foo"), config }
    }

    pub fn run_hook(&self, input: &str) -> Result<Summary, Error> {
        self.check_commits(&parse_range(input)?)
    }

    fn check_commits(&self, spec: &str) -> Result<Summary, Error> {
        let range = self.repo.revparse(spec).with_context(|| format!("invalid range '{}'", spec))?;
        let remote_commit = self.repo.find_commit(range.from().unwrap().id())?;
        let local_commit  = self.repo.find_commit(range.to().unwrap().id())?;

        let diff = self.repo.diff_tree_to_tree(
            Some(&remote_commit.tree().unwrap()),
            Some(&local_commit.tree().unwrap()),
            Some(&mut DiffOptions::new()),
        ).unwrap();

        self.config.check(diff)
    }
}

fn parse_range(input: &str) -> Result<String, Error> {
    let args = input.split_whitespace()
        .filter_map(|arg| arg.parse().ok())
        .take(4)
        .collect::<Vec<String>>();

    if let [_, local_oid, _, remote_oid] = &args[..] {
        return Ok(create_range(local_oid, remote_oid));
    }

    anyhow::bail!("Expected: local_ref local_oid remote_ref remote_oid\n  Actual: '{}'", args.join(" "))
}

fn create_range(local_oid: &str, remote_oid: &str) -> String {
    if *local_oid == zero_oid() {           // allow push / skip commits
        String::new()
    } else if *remote_oid == zero_oid() {   // new branch, examine all commits
        local_oid.to_string()
    } else {                                // examine all commits
        format!("{}..{}", remote_oid, local_oid)
    }
}

impl Config {
    fn check(&self, diff: Diff) -> Result<Summary, Error> {
        let mut output = String::new();

        let count = diff.deltas()
            .filter( |delta| self.countable(delta))
            .inspect(|delta| Config::append(&mut output, delta))
            .count();

        output.insert_str(0, &format!(">>> inspected changes: {} of {}", count, diff.deltas().count()));

        if count <= self.change_threshold { return Ok(Summary { contents: output }); }
        anyhow::bail!("This change set is too large: push fewer changes more frequently instead.\n{}", output)
    }

    fn countable(&self, delta: &DiffDelta) -> bool {
        vec![Delta::Added, Delta::Modified, Delta::Deleted].contains(&delta.status())
            && self.ignored_patterns.iter().all(|p|
                !p.matches(delta.new_file().path().unwrap().to_string_lossy().as_ref())
            )
    }

    fn append(output: &mut String, delta: &DiffDelta) {
        write!(output, "\n>>> {}\t{}",
               format!("{:?}", delta.status()).chars().next().unwrap(),
               delta.new_file().path().unwrap().to_string_lossy()
        ).unwrap()
    }
}

#[cfg(test)]
mod unit_tests;
