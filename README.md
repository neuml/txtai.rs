# txtai: AI-powered search engine for Rust

[![Version](https://img.shields.io/github/release/neuml/txtai.rs.svg?style=flat&color=success)](https://github.com/neuml/txtai.rs/releases)
[![GitHub Release Date](https://img.shields.io/github/release-date/neuml/txtai.rs.svg?style=flat&color=blue)](https://github.com/neuml/txtai.rs/releases)
[![GitHub issues](https://img.shields.io/github/issues/neuml/txtai.rs.svg?style=flat&color=success)](https://github.com/neuml/txtai.rs/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/neuml/txtai.rs.svg?style=flat&color=blue)](https://github.com/neuml/txtai.rs)

[txtai](https://github.com/neuml/txtai) builds an AI-powered index over sections of text. txtai supports building text indices to perform similarity searches and create extractive question-answering based systems. txtai also has functionality for zero-shot classification.

This repository contains Rust bindings for the txtai API. Full txtai functionality is supported.

## Installation

Add the following lines to your project `Cargo.toml` file:

```toml
[dependencies]
txtai = { version = "1.0" }
tokio = { version = "0.2", features = ["full"] }
```

This adds txtai as a dependency as well as tokio given txtai uses async io.

## Examples
The examples directory has a series of examples that give an overview of txtai. See the list of examples below.

| Example     |      Description      |
|:----------|:-------------|
| [Introducing txtai](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/main.rs#L29) | Overview of the functionality provided by txtai |
| [Extractive QA with txtai](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/main.rs#L73) | Extractive question-answering with txtai |
| [Labeling with zero-shot classification](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/main.rs#L123) | Labeling with zero-shot classification |

txtai.rs connects to a txtai api instance. See [this link](https://github.com/neuml/txtai#api) for details on how to start a new api instance.

Once an api instance is running, do the following to run the examples.

```
git clone https://github.com/neuml/txtai.rs
cd txtai.rs/examples/demo
cargo run
```
