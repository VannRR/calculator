# Calculator

This is a simple integer calculator written in Rust, that uses bitwise logic and the shunting yard algorithm.

## Overview

This calculator is written in [Rust](https://github.com/rust-lang/rust), uses bitwise logic to perform calculations
and parses input using the shunting yard algorithm.
The GUI is developed with [gtk4-rs](https://github.com/gtk-rs/gtk4-rs) and
[leptos](https://github.com/leptos-rs/leptos) is used for signals and updates.

The goal of this project was to learn Rust and how to create native GUI applications using
libraries that feel familiar to those with experience in browser-based JavaScript.

The logic module is heavily commented and could potentially serve as a helpful reference for understanding binary mathematical operations and mathematical equation parsing.

## Download

You can download the latest linux x86_64 release of the calculator from the [releases page](https://github.com/vannrr/calculator/releases).
Note you also need to install the your distributions `gtk4` package to run the binary.

## Build

To build the calculator from source installing `rust`, `cargo` and `gtk4` are required, then follow these steps:

```shell
git clone https://github.com/vannrr/calculator
cd ./calculator
cargo build --release

```

## License

This software is distributed under the
[MIT license](https://opensource.org/licenses/MIT).
