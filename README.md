# Rust Snippets.
This repository contains a couple of Rust packages that I have created for learning purposes.  
It is heavily inspired by the book [Programming Rust, 2nd Edition](https://learning.oreilly.com/library/view/programming-rust-2nd/9781492052586/) 

----------
## What I learned in each package

### Mandlebrot
<img src="./mandelbort/mandle.png" width="75%" />

* `loop` statement for infinite loop.
* `?` syntax for cleaner `Result, Err` checking.
* `image` crate for PNGEncoding.
* `match` statement for tuple based decision making.
* Variable name based direct attribute binding.
* `Optional` return statements to either return `Some` or `None`.
* `vec![num, size]` macro initialize a collection with initial value.
* SAFE concurrent multithreaded programming using `crossbeam::scope`.
* `move` keyword for closures.

### Quickreplace
* Creating a CLI program.
* Basic I/O and data processing.
* Handling corrupt arguments.
* `#[derive(Debug)]` attribute for straightforward debug printing.
* Using `regex` as a crate to manipulate strings.
* Colorizing stdout by `text-colorizer` crate.


### CLI Args
* Using standard library.
* Passing arguments and parsing them.
* Rust's `Result` handling style.
* The logic of *Borrowing* and *Derefrencing*.
* Brief intro to `Trait`.

### Actix GCD
* Rust `closures` and how they relate to functions.
* Using online crates with cargo.
* Hosting an HTTP server with Actix.
* Deserializing POST form parameters and call a fn.
* `format!()` macro to manipulate strings.

### Hello
* Variable declarations. (`let t`, `let t: u64`)
* Function call and mutable syntax. (`fn foo(mut a: u32)`)
* Macro declarations. (`whatever!()`)
* Return values with no keyword. (`n`)
* Machine specific word size. (`isize`, `usize`)
* Function attribute & Unit testing. (`#[test]`)

