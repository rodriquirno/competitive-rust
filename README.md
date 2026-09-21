# competitive-rust

[![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org/)

A collection of **competitive programming** solutions written in **Rust**.

## Table of Contents

* [Overview](#overview)
* [Directory Structure](#directory-structure)
* [Prerequisites](#prerequisites)
* [Usage](#usage)

## Overview

Solved problems from online judges, one judge per top-level folder. For now the only
judge here is [Codeforces](https://codeforces.com/); see
[codeforces/README.md](codeforces/README.md) for the full index of solved problems.

Every solution is a standalone binary that reads from stdin and writes to stdout, so
the file can be pasted straight into the judge.

## Directory Structure

```
├── codeforces          # Codeforces solutions (a single Cargo project)
│   ├── src/bin         # one file per problem, named <contest><index>.rs
│   └── README.md       # index of solved problems
├── .gitignore
└── README.md
```

Each source file starts with a comment holding the problem id and title, e.g.
`// 115A. Party`.

## Prerequisites

* **Rust**: `rustc` and `cargo` 1.85+ (the crates use edition 2024)

## Usage

Run a solution by its file name (without the `.rs` extension):

```sh
cd codeforces
cargo run --release --bin 1627c              # reads from stdin
cargo run --release --bin 1627c < input.txt  # or feed it a file
```
