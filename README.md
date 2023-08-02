# kid

[![Rust](https://github.com/fujianbang/kid/actions/workflows/rust.yml/badge.svg)](https://github.com/fujianbang/kid/actions/workflows/rust.yml)

**Kid** is a toy Rust-written interpreter.

## Features

* Written in pure Rust
* C-like syntax
* Variable bindings
* Integers and booleans
* Arithmetic expressions
* Built-in functions
* First-class and higher-order functions
* Closures
* String data structure
* Array data structure
* Hash data structure

## Usage

```bash
let five = 5;
let ten = 10;

let add = fun(a, b) {
    return a + b;
};

let result = add(five, ten);
```