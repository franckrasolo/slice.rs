# `slice.rs` <img align="right" width="160" height="120" src="https://github.com/franckrasolo/slice.rs/raw/633588c6041784ce036c8ec0f60c8698bc171973/.github/assets/slicer.png">

[![CI](https://github.com/franckrasolo/slice.rs/workflows/CI/badge.svg)](https://github.com/franckrasolo/slice.rs/actions?query=workflow%3A%22CI%22)
[![Security Audit](https://github.com/franckrasolo/slice.rs/workflows/Security%20Audit/badge.svg)](https://github.com/franckrasolo/slice.rs/actions?query=workflow%3A%22Security+Audit%22)
[![dependabot](https://badgen.net/github/dependabot/franckrasolo/slice.rs?icon=dependabot&label=Dependabot&labelColor=30373d&cache=300)](https://github.com/franckrasolo/slice.rs/network/updates)
[![Apache License 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=flat&logo=apache&labelColor=30373d)](https://github.com/franckrasolo/slice.rs/blob/main/LICENSE)

#### Improve your *trunk-based development* game with `pre-commit` and `pre-push` Git hooks!

These hooks are meant to encourage:
* committing fewer changes
* constructing a narrative through [micro commits](https://lucasr.org/2011/01/29/micro-commits/) leaving a trail of breadcrumbs
* more frequent pushes of small change sets to the default branch: `main` | `trunk`
* more frequent integration
* working at a sustainable pace

They rather tyrannically enforce ideas discussed by in
[Limbo on the Cheap](https://medium.com/@kentbeck_7670/limbo-on-the-cheap-e4cfae840330) and work equally
well when combined with the [TCR](https://medium.com/@kentbeck_7670/test-commit-revert-870bbd756864) workflow.

Incremental development, also discussed in
[Elephant Carpaccio](https://web.archive.org/web/20140329231444/http://alistair.cockburn.us/Elephant+Carpaccio)
and [Elephant Carpaccio V2](https://medium.com/@matteoregazzi/elephant-carpaccio-v2-ba984640ce88), suggests
aiming for thinner and thinner slices as units of work (see *fig. 1*) that must be propagated at the earliest
opportunity to other developers and validated through a continuous integration pipeline.

![Slice Sizes](https://github.com/franckrasolo/slice.rs/raw/633588c6041784ce036c8ec0f60c8698bc171973/.github/assets/scale.png)

*fig. 1 from the
[Elephant Carpaccio Exercise Facilitation Guide](https://docs.google.com/document/d/1TCuuu-8Mm14oxsOnlk8DqfZAA1cvtYu9WGv67Yj_sSk/pub)
by Henrik Kniberg & Alistair Cockburn, 2013*

## Installation

TBD
