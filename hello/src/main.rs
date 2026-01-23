// u64, i64 -> Unsigned & Signed 64 bit integers.
// isize, usize -> Machine's word length (i.e 32-bit, 64-bit etc)
// 'mut' allows the fn body to modify the parameter.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // '!' indicates macro usage, not a fn.
    // When true, the program 'panic'.
    assert!(n != 0 && m != 0);
    while m != 0{
        if m < n {
            // declare a 'local' variable.
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    // an expression with no ';'
    // This is the return value !
    n
}

fn main() {
    println!("Hello World!");
    // declare a 'local' variable.
    // Force variable's data type.
    let result: u64 = gcd(15, 5);
    println!("{result}");
}
