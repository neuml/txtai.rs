# txtai: Build AI-powered semantic search applications in Rust

[![Version](https://img.shields.io/github/release/neuml/txtai.rs.svg?style=flat&color=success)](https://github.com/neuml/txtai.rs/releases)
[![GitHub Release Date](https://img.shields.io/github/release-date/neuml/txtai.rs.svg?style=flat&color=blue)](https://github.com/neuml/txtai.rs/releases)
[![GitHub issues](https://img.shields.io/github/issues/neuml/txtai.rs.svg?style=flat&color=success)](https://github.com/neuml/txtai.rs/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/neuml/txtai.rs.svg?style=flat&color=blue)](https://github.com/neuml/txtai.rs)

[txtai](https://github.com/neuml/txtai) executes machine-learning workflows to transform data and build AI-powered semantic search applications.

This repository contains Rust bindings for the txtai API. Full txtai functionality is supported.

## Installation

Add the following lines to your project `Cargo.toml` file:

```toml
[dependencies]
txtai = { version = "5.4" }
tokio = { version = "0.2", features = ["full"] }
```

This adds txtai as a dependency as well as tokio given txtai uses async io.

## Examples
The examples directory has a series of examples that give an overview of txtai. See the list of examples below.

| Example     |      Description      |
|:----------|:-------------|
| [Introducing txtai](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/embeddings.rs) | Overview of the functionality provided by txtai |
| [Extractive QA with txtai](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/extractor.rs) | Extractive question-answering with txtai |
| [Labeling with zero-shot classification](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/labels.rs) | Labeling with zero-shot classification |
| [Pipelines and workflows](https://github.com/neuml/txtai.rs/blob/master/examples/demo/src/pipelines.rs) | Pipelines and workflows |

txtai.rs connects to a txtai api instance. See [this link](https://github.com/neuml/txtai#api) for details on how to start a new api instance.

Once an api instance is running, do the following to run the examples.

```
git clone https://github.com/neuml/txtai.rs
cd txtai.rs/examples/demo
cargo run
```
