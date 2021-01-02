use std::path::Path;

use anyhow::Error;
use glob::Pattern;
use spectral::*;
use spectral::prelude::{ResultAssertions, StrAssertions};
use test_case::test_case;

use crate::{Config, zero_oid};
use crate::pre_push::{self, PrePush, Summary};

#[test_case(&zero_oid(), "903f52874", ""; "when local branch is deleted")]
#[test_case("4e5543acf", &zero_oid(), "4e5543acf"; "when remote branch does not exist yet")]
#[test_case("4e5543acf", "903f52874", "903f52874..4e5543acf"; "when revisions are not zero")]
fn created_range(local_oid: &str, remote_oid: &str, expected: &str) {
    let actual = pre_push::create_range(local_oid, remote_oid);
    assert_that!(actual).is_equal_to(expected.to_string());
}

#[test]
fn parse_range_when_input_is_empty() {
    let input = "";
    let error = pre_push::parse_range(input).unwrap_err();
    asserting!("parsing error")
        .that(&format!("{}", error)).is_equal_to(parsing_error_for(input));
}

#[test]
fn parse_range_when_input_lacks_some_values() {
    let input = "a b";
    let error = pre_push::parse_range(input).unwrap_err();
    asserting!("parsing error")
        .that(&format!("{}", error)).is_equal_to(parsing_error_for(input));
}

#[test]
fn parse_range_when_input_has_all_expected_values() {
    let spec = pre_push::parse_range("a b c d").unwrap();
    asserting!("parsed range").that(&spec).is_equal_to("d..b".to_string());
}

#[test]
fn parse_range_when_input_has_additional_values() {
    let spec = pre_push::parse_range("a b c d e f").unwrap();
    asserting!("parsed range").that(&spec).is_equal_to("d..b".to_string());
}

fn parsing_error_for(input: &str) -> String {
    format!("Expected: local_ref local_oid remote_ref remote_oid\n  Actual: '{}'", input)
}

#[test]
fn run_hook_when_local_revision_is_not_found() {
    let local_oid  = "???";
    let remote_oid = "764ecf6ed4de341203b9c9af94db4ac279c087fe";

    let error_message = run_hook(local_oid, remote_oid).unwrap_err().to_string();

    asserting!("push rejected")
        .that(&error_message)
        .is_equal_to(format!("invalid range '{}..{}'", remote_oid, local_oid));
}

#[test]
fn run_hook_when_too_many_changes_are_pushed() {
    let local_oid  = "764ecf6ed4de341203b9c9af94db4ac279c087fe";
    let remote_oid = "c89ad800371ebf5300212ddd0523b14534fc99cc";

    let error_message = run_hook(local_oid, remote_oid).unwrap_err().to_string();

    asserting!("push rejected")
        .that(&error_message)
        .contains("change set is too large");
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
        change_threshold: 3
    };
    let input = format!("{} {} {} {}", "local_ref", local_oid, "remote_ref", remote_oid);
    PrePush::new(Path::new(&repo_path), config).run_hook(&input)
}
