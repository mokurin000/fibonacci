use std::ops::{Mul, Sub};

use gmp::mpz::Mpz;
use rayon::join;

fn fib(x: Mpz) -> Mpz {
    if x < Mpz::from(3) {
        Mpz::from(1)
    } else {
        if !x.is_multiple_of(&Mpz::from(2)) {
            let n = (x + 1) / 2;
            let m = n.clone();
            let (a, b) = join(|| fib(m).pow(2), || fib(n - 1).pow(2));
            a + b
        } else {
            let n = x / 2;
            let m = n.clone();
            let (a, b) = join(|| fib(m), || fib(n.sub(&Mpz::from(1))));
            (b.mul(Mpz::from(2)) + &a) * a
        }
    }
}

fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();

    let result = fib(n);
    println!("{result}");
}
