<div align="center">
  <h1>Yew and Bulma</h1>

[![Crate](https://img.shields.io/crates/v/yew-and-bulma.svg)](https://crates.io/crates/yew-and-bulma)
[![Build status](https://img.shields.io/github/actions/workflow/status/filipdutescu/yew-and-bulma/ci.yml?branch=main)](https://github.com/filipdutescu/yew-and-bulma/actions)
[![Docs](https://img.shields.io/badge/docs.rs-yew--and--bulma-green)](https://docs.rs/yew-and-bulma/)
![Licenses](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)

  <h3> Bulma CSS components for Yew</h3>

  <h4>
    <a href="https://docs.rs/yew-and-bulma/">Documentation</a>
    <span> | </span>
    <a href="https://github.com/filipdutescu/yew-and-bulma/tree/master/examples">Examples</a>
    <span> | </span>
    <a href="https://github.com/filipdutescu/yew-and-bulma/blob/master/CHANGELOG.md">Changelog</a>
  </h4>
</div>

This crate provides [Bulma CSS][bulma] components made to be used with the
[Yew][yew] framework. It aims to make an easy, as intuitive as possible
development experience for integrating [Bulma][bulma] into your [Yew][yew]
frontends.

Generally speaking, it aims to provide a Rust API for ideally* all elements,
components, helpers etc. that you would be able to use in CSS/HTML or other
frontend frameworks, such as Angular or React.

> _* It might not be possible to expose everything in the same manner as
with JavaScript, but wherever it is, this crate will try and implement them._

# Table of contents

1. [Features](#features)
2. [Usage](#usage)
3. [Minimum supported Rust version](#minimum-supported-rust-version)
4. [Contributing](#contributing)
5. [License](#license)

# Features

* High-level APIs for creating [Bulma][bulma] components and using Bulma helpers
  with [Yew][yew]
* Ready to use utilities for custom component creation
* Flexible components that can be combined together. Whatever is shown in the
  [Bulma][bulma] documentation can be achieved with this crate.

# Usage

Since it is in the early stages of development, no complete example is made
yet.

# Minimum Supported Rust Version (MSRV)

Current MSRV is [1.60](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html).

# Contributing

Since it is in the early stages of development, no contributing guidelines are
present.

# License

[bulma]: https://bulma.io
[yew]: https://yew.rs
