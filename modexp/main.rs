//! Command-line modular exponentation tool
//!
//! Mikayla Maki 2021

use std::env;

///The program entry point, parses the command line arguments
fn main() {

    let args: Vec<u64> = env::args()
        .skip(1) //The first item is always the name of program
        .map(|arg| arg.parse().unwrap_or_else(|_| error()))
        .collect();

    if args.len() < 3 || args[2] == 0 {
        error();
    }

    println!("{}", modexp(args[0], args[1], args[2]));
}

///Implements the fast modular exponentiation algorithm: x^y % m
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut y = y; //For consistency
    let mut x: u128 = u128::try_from(x).unwrap();
    let mut z: u128 = 1;

    while y > 0 {
        if y % 2 == 1 {
            z = (z * x) % m as u128;
        }
        y /= 2;
        x = (x * x) % m as u128;
    }

    u64::try_from(z).unwrap()
}

///TODO
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}
