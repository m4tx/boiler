boiler
======

[![Rust Build Status](https://github.com/m4tx/boiler/workflows/Rust%20CI/badge.svg)](https://github.com/m4tx/boiler/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/github/license/m4tx/boiler)](https://github.com/m4tx/boiler/blob/master/LICENSE)
[![codecov](https://codecov.io/gh/m4tx/boiler/branch/master/graph/badge.svg)](https://codecov.io/gh/m4tx/boiler)

Boiler is a highly opinionated CLI tool for creating and updating boilerplate files for your projects. It is capable of automatically detecting various different metadata from the project files, such as the programming languages used, frameworks, and even specific features of git, and then generating the boilerplate files based on that information. The boilerplate includes, but is not limited to, CI pipelines, pre-commit hooks, and configuration files.

## Building

The project is written in Rust and uses Cargo build system.

```shell
cargo build --release
```

## Running

Simply run the binary in the root directory of your project. It will automatically detect the project type and generate the boilerplate files in the current directory.

For more information, run `boiler --help`.

## Developing

### `pre-commit`

We encourage contributors to use predefined [`pre-commit`](https://pre-commit.com/)
hooks — to install them in your local repo, make sure you have `pre-commit`
installed and run

```shell
pre-commit install
```
