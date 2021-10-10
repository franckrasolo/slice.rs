## Backlog

- replace glob with globset
- add deserialisation of glob patterns
- pre-commit hook
    * [x] pick a name for the binary
    * [ ] check the number of lines changed in a single commit
    * [ ] check the number of files changed in a single commit
- find ways to easily install these hooks into a local repository, ideally automatically
- add integration tests for cross-compiled binaries
- run integration tests of cross-compiled binaries as part of the CD pipeline
- make `cargo test` produce JUnit XML reports
- run a [TCR](https://medium.com/@kentbeck_7670/test-commit-revert-870bbd756864) workflow with [cargo-watch](https://github.com/passcod/cargo-watch)

## Doing


## Done

- figure out how to organise, layout and configure a Cargo monorepo
- figure out how to produce multiple binaries
- pre-push hook
    * [x] pick a name for the binary
    * [ ] check the number of files changed across commits
    * [ ] check the number of lines changed across commits?
- add an exclusion mechanism with sane defaults
    * [x] plain text: `*.conf`, `*.csv`, `*.dhall`, `*.md`, `*.nix`, `*.properties`, `*.txt`
    * [x] specific files: `README`, `.gitlab-ci.yml`
    * [x] scripts: `*.gradle`, `*.sh`, `*.bash`, `*.zsh`, `Makefile`, `justfile`
    * [x] directories: `.github`, `.teamcity/**`
- add configuration for automatic dependency updates
- set up [ShiftLeft Scan](https://slscan.io/) as a GitHub action
- set up [cargo-audit](https://github.com/RustSec/cargo-audit) as a GitHub action
- add GitHub Action to integrate across supported OS/architectures
    > instruction set architectures / operating systems
    * [x] `x86_64` Linux: Ubuntu 20.04 (LTS)
    * [x] `x86_64` Linux: Ubuntu 18.04 (LTS)
    * [x] `x86_64` macOS Big Sur
    * [x] `x86_64` macOS Catalina
    * [x] `x86_64` macOS High Sierra ([self-hosted](https://github.com/actions/virtual-environments/issues/2247))
- add GitHub Action to cross-compile and publish binaries
    > instruction set architectures / operating systems
    * [x] `x86_64` Linux: Ubuntu 18.04 (LTS)
    * [x] `x86_64` macOS High Sierra
    * [x] `AArch64` Linux: Ubuntu 18.04 (LTS)
- add release step to compress binaries

## Blocked

- add GitHub Action to cross-compile and publish binaries
    > instruction set architectures / operating systems
    * [ ] `x86_64` macOS Catalina / Big Sur
    * [ ] `AArch64` M1 macOS Big Sur
    * [ ] `Universal` macOS Big Sur
