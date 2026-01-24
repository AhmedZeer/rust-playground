// Trait !
// Traits must be included to be 
// uesd from implemented methods.
// It is like interface
use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0{
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

// No need to return from main.
// If main returns anything the prgoram
// ran successfully.
fn main() {

    // Dynamic array
    let mut numbers = Vec::new();

    // This trait does not return the value
    // immediately. Instead, it returns a 
    // `Result` object, which is either
    // Ok(val) or Err(e).
    // Rust does not have exceptions!
    // Either panic or Result.
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg)
            .expect("Oopsie"));
    }

    if numbers.len() == 0{
        eprintln!("USAGE: gcd number ...");
        std::process::exit(1);
    }

    // We borrow an element from numbers
    // vector in each iteration. This means
    // that we do not own the element.
    let mut d = numbers[0];
    for borrower in &numbers[1..] {
        d = gcd(d, *borrower);
    }
}
