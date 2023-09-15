set dotenv-load := true

_targets:
  @just --list --unsorted --list-heading $'Available targets:\n' --list-prefix "  "

# updates the top-level flake lock file
@update:
  nix flake update

# runs all tests
@check:
  git submodule update --init
  cargo test --package slicers --lib pre_push::unit_tests
