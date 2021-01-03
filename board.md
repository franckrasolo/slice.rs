## Backlog

- pre-commit hook
    * [ ] pick a name for the binary
    * [ ] check the number of lines changed in a single commit
    * [ ] check the number of files changed in a single commit
- find ways to easily install these hooks into a local repository, ideally automatically
- make `cargo test` produce JUnit XML reports
- run a [TCR](https://medium.com/@kentbeck_7670/test-commit-revert-870bbd756864) workflow with [cargo-watch](https://github.com/passcod/cargo-watch)

## Doing

- set up [cargo-audit](https://github.com/RustSec/cargo-audit) as GitHub action 
- add GitHub Action to cross-compile binaries
    * [ ] `x86_64` macOS Catalina / Big Sur
    * [ ] `Universal` binary for macOS Big Sur
    * [ ] `x86_64` Linux
    * [ ] `x86_64` macOS High Sierra
    * [ ] `AArch64` M1 macOS Big Sur
    * [ ] `AArch64` Linux
- add an exclusion mechanism with sane defaults
    * [ ] plain text: `*.conf`, `*.csv`, `*.dhall`, `*.md`, `*.nix`, `*.properties`, `*.txt`
    * [ ] specific files: `README`, `.gitlab-ci.yml`
    * [ ] scripts: `*.gradle`, `*.sh`, `*.bash`, `*.zsh`, `Makefile`, `justfile`
    * [ ] directories: `.github`, `.teamcity/**`

## Done

- figure out how to organise, layout and configure a Cargo monorepo
- figure out how to produce multiple binaries
- pre-push hook
    * [ ] pick a name for the binary
    * [ ] check the number of files changed across commits
    * [ ] check the number of lines changed across commits?
- add configuration for automatic dependency updates
