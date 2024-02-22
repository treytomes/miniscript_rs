# miniscript_rs

**A Rust implementation of the MiniScript language.**

I will be following along with the Crafting Interpreters book as I build this.

The interpreter will be built in 2 stages:

## Stage 1

Build a recursive descent parser that will execute MiniScript programs.

## Stage 2

ASTs will be compiled to WASM and executed there.

More details to follow as I figure them out.

I'm not fully committed to targetting Rust for the IR.  Here are 2 other options:
1. https://cranelift.dev/
2. https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html

Just learned about CraneLift recently.  It looks like it might be a more modern tech.
Bonus is that either of these could probably still compile to either native or WASM if I wanted.

# TODO

- https://docs.rs/miette/latest/miette/
    - miette is a diagnostic library for Rust. It includes a series of traits/protocols that allow you to hook into its error reporting facilities, and even write your own error reports!
 
- https://docs.rs/thiserror/latest/thiserror/
    - This library provides a convenient derive macro for the standard library’s std::error::Error trait.
