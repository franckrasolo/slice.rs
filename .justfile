set dotenv-load := true

_targets:
  @just --list --unsorted --list-heading $'Available targets:\n' --list-prefix "  "

# updates the top-level flake lock file
@update:
  nix flake update --commit-lock-file --commit-lockfile-summary "update Nix flake inputs"

# runs all tests
@check:
  cargo test --package slicers --lib pre_push::unit_tests
