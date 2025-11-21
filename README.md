<p align="center">
    <img src="https://raw.githubusercontent.com/neuml/txtai/master/logo.png"/>
</p>

<p align="center">
    <b>Rust client for txtai</b>
</p>

<p align="center">
    <a href="https://github.com/neuml/txtai.rs/releases">
        <img src="https://img.shields.io/github/release/neuml/txtai.rs.svg?style=flat&color=success" alt="Version"/>
    </a>
    <a href="https://github.com/neuml/txtai.rs/releases">
        <img src="https://img.shields.io/github/release-date/neuml/txtai.rs.svg?style=flat&color=blue" alt="GitHub Release Date"/>
    </a>
    <a href="https://github.com/neuml/txtai.rs/issues">
        <img src="https://img.shields.io/github/issues/neuml/txtai.rs.svg?style=flat&color=success" alt="GitHub Issues"/>
    </a>
    <a href="https://github.com/neuml/txtai.rs">
        <img src="https://img.shields.io/github/last-commit/neuml/txtai.rs.svg?style=flat&color=blue" alt="GitHub Last Commit"/>
    </a>
</p>

[txtai](https://github.com/neuml/txtai) is an all-in-one AI framework for semantic search, LLM orchestration and language model workflows.

This repository contains Rust bindings for the txtai API.

## Installation

Add the following lines to your project `Cargo.toml` file:

```toml
[dependencies]
txtai = { version = "9.2" }
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

txtai.rs connects to a txtai api instance. See [this link](https://neuml.github.io/txtai/api/) for details on how to start a new api instance.

Once an api instance is running, do the following to run the examples.

```
git clone https://github.com/neuml/txtai.rs
cd txtai.rs/examples/demo
cargo run
```
