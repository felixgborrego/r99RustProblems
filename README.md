# _R-99: Ninety-Nine Rust Problems_

This is a pet project to learn Rust while I'm trying to code my own implementation of the [99 Scala problems](https://github.com/felixgborrego/s99ScalaProblems).

You can find more information at [The 99 Problems](https://www.ic.unicamp.br/~meidanis/courses/mc336/2006s2/funcional/L-99_Ninety-Nine_Lisp_Problems.html)

## Testing

This project uses standard cargo, just install cargo and run:

`cargo test`

## Project Setup

* Install Rust and addons

After installing rust up following the [official docs](https://www.rust-lang.org/tools/install).

```
rustup update
rustup component add rustfmt
rustup component add clippy

```

* Code Lint and Format

```
cargo clippy --fix
cargo fmt
```
