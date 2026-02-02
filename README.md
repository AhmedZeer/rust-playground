# Rust Snippets.
This repository contains a couple of Rust packages that I have created for learning purposes.  
* It is heavily inspired by the book [Programming Rust, 2nd Edition](https://learning.oreilly.com/library/view/programming-rust-2nd/9781492052586/) 
* Random guy YouTube video [Rust Programming Full Course](https://www.youtube.com/watch?v=rQ_J9WH6CGk)
* Holy book [Rust Official Book](https://doc.rust-lang.org/book/)

----------
## What I learned in each package
### Hello
* Variable declarations. (`let t`, `let t: u64`)
* Function call and mutable syntax. (`fn foo(mut a: u32)`)
* Macro declarations. (`whatever!()`)
* Return values with no keyword. (`n`)
* Machine specific word size. (`isize`, `usize`)
* Function attribute & Unit testing. (`#[test]`)

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

### Basic Of Rust
* Shadowing
* Using Cargo Dependencies
* `rand` library
* Basic loop and conditions syntax
* and other cool stuffs