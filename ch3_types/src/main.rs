fn _build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}
// these typing is Duck Typing: "if it quacks like a duck, it’s a duck." :d
fn _build_vector_duck() -> Vec<i16> {
    let mut v = Vec::new();     // rust can inference type of v and their elements
    v.push(10);                 // do not need write explicitly each type
    v.push(20);
    v
}

// Machine word	   usize	  isize <-- its run on native machine address size like 32 or 64 
// The prefixes 0x, 0o, and 0b designate hexadecimal, octal, and binary literals.

#[test]
fn test_type_cast() 
{
    assert_eq!(   10_i8  as u16,    10_u16); // in range
    assert_eq!( 2525_u16 as i16,  2525_i16); // in range

    assert_eq!(   -1_i16 as i32,    -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation."
    assert_eq!( 1000_i16 as  u8,   232_u8);
    assert_eq!(65535_u32 as i16,    -1_i16);

    assert_eq!(   -1_i8  as u8,    255_u8);
    assert_eq!(  255_u8  as i8,     -1_i8);

    assert_eq!(2_u16.pow(4), 16);            // exponentiation
    assert_eq!((-4_i32).abs(), 4);           // absolute value
    assert_eq!(0b101101_u8.count_ones(), 4); // population count
}

#[test]
fn test_checked_wrapping_saturating_overflowing()
{
    // The sum of 10 and 20 can be represented as a u8.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None);

    // Do the addition; panic if it overflows.
    let x: i32 = 100_i32;
    let y = 100;
    let _sum = x.checked_add(y).unwrap();

    // Oddly, signed division can overflow too, in one particular case.
    // A signed n-bit type can represent -2ⁿ⁻¹, but not 2ⁿ⁻¹.
    assert_eq!((-128_i8).checked_div(-1), None);

    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2¹⁶.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift
    // of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
    // There are no saturating division, remainder, or bitwise shift methods.

    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
} 

#[test]
fn test_float_bool()
{
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);

    assert_eq!(false as i32, 0);
    assert_eq!(true  as i32, 1);

    // assert_eq!(1 as bool, true); Cannot cast i32 to bool in Rust use below
    // assert_eq!(0 as bool, false);

    assert_eq!(1 != 0, true);
    assert_eq!(0 != 0, false);
}

#[test]
fn test_char()
{
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncated to eight bits, signed

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('1'.to_digit(2), Some(1));
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

// Tuples allow only constants as indices, like t.4. You can’t write t.i or t[i] to get the ith element.

#[test]
fn test_tuple()
{
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // upper one more readable 
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

#[test]
fn test_array()
{
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort(); // if you want use it chaos must be mut
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vectors()
{
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    // Reasonable yet disappointing:
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

}
