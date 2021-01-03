use std::path::Path;

use anyhow::Error;
use glob::Pattern;
use spectral::prelude::{ResultAssertions, StrAssertions};
use spectral::*;
use test_case::test_case;

use crate::pre_push::{self, PrePush, Summary};
use crate::{zero_oid, Config};

#[test_case(&zero_oid(), "903f52874", ""; "when local branch is deleted")]
#[test_case("4e5543acf", &zero_oid(), "4e5543acf"; "when remote branch does not exist yet")]
#[test_case("4e5543acf", "903f52874", "903f52874..4e5543acf"; "when revisions are not zero")]
fn created_range(local_oid: &str, remote_oid: &str, expected: &str) {
    let actual = pre_push::create_range(local_oid, remote_oid);
    assert_that!(actual).is_equal_to(expected.to_string());
}

#[test_case("";    "when input is empty")]
#[test_case("a b"; "when input lacks some args")]
fn parse_range_errors(input: &str) {
    let error = pre_push::parse_range(input).unwrap_err();
    assert_that!(format!("{}", error)).ends_with(format!("Actual: '{}'", input).as_str());
}

#[test_case("a b c d",     "d..b"; "when input has all expected args")]
#[test_case("a b c d e f", "d..b"; "when input has additional args")]
fn parsed_range(input: &str, expected: &str) {
    let spec = pre_push::parse_range(input).unwrap();
    assert_that!(spec).is_equal_to(expected.to_string());
}

#[test_case("???", "764ecf6ed4de341203b9c9af94db4ac279c087fe",
            "invalid range"; "when local revision is not found"
)]
#[test_case("764ecf6ed4de341203b9c9af94db4ac279c087fe", "c89ad800371ebf5300212ddd0523b14534fc99cc",
            "change set is too large"; "when too many changes are pushed"
)]
fn push_rejected(local_oid: &str, remote_oid: &str, expected: &str) {
    let error_message = run_hook(local_oid, remote_oid).unwrap_err().to_string();
    assert_that!(error_message).contains(expected);
}

#[test]
fn run_hook_when_few_changes_are_pushed() {
    let local_oid  = "764ecf6ed4de341203b9c9af94db4ac279c087fe";
    let remote_oid = "971595ab3f742abcf8c8cf839cff2a46a4d95feb";

    assert_that!(run_hook(local_oid, remote_oid)).is_ok();
}

fn run_hook(local_oid: &str, remote_oid: &str) -> Result<Summary, Error> {
    let repo_path = format!("{}/tests/resources/test-repo", env!("CARGO_MANIFEST_DIR"));
    let config = Config {
        ignored_patterns: vec![Pattern::new("*.yml").unwrap()],
        change_threshold: 3,
    };
    let input = format!("{} {} {} {}", "local_ref", local_oid, "remote_ref", remote_oid);
    PrePush::new(Path::new(&repo_path), config).run_hook(&input)
}
