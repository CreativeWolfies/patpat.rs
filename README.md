# PatPat.rs

An (WIP) interpreter written in Rust for the PatPat language.

The original documentation can be found [here](https://github.com/adri326/patpat.js/).
This documentation should be up-to-date for the majority of the things described.

## Installation

Clone this repository and build it with `cargo`:

```sh
git clone https://github.com/CreativeWolfies/patpat.rs
cd patpat.rs
cargo build
```

## Testing the language

You can try one of the example scripts by running:

```sh
cargo run examples/fibonacci.patpat
```

Automated tests have been set up and can be run with `cargo test`.

## How it works

Your program is first lexically analysed; this is done by the `src/parser/` section.
It will first be separated into tokens and then be assembled by the `src/parser/construct/` section.
An Abstract Syntactical Tree (AST) will be produced, if everything went well.

Once that is done, its references (variables, functions, structs, ...) will be resolved by the `src/ast/resolve/` section.
This process returns a Resolved AST (RAST), which will be ready to be interpreted.

The interpreter (`src/interpreter`) works on the RAST. It operates with a stack of Contexes, each holding the values of variables at a given depth in the code.

`src/internal` contains the internal functions (`#if`, etc.). These are hard-coded functions, which are exposed as regular functions (patterns).
