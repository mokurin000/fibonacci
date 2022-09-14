pub type IndexType = u64; // this is currently much larger than your mem/cpu limitation

use std::ops::Mul;

use gmp::mpz::Mpz;
use rayon::join;

fn _no_parralel_fib(x: IndexType) -> (Mpz, Mpz) {
    if x < 3 {
        (Mpz::from(0), Mpz::from(1))
    } else {
        if x % 2 == 1 {
            let n = (x + 1) / 2;
            (fib(n).pow(2), fib(n - 1).pow(2))
        } else {
            let n = x / 2;
            let (a, b) = (fib(n), fib(n - 1));
            (b.mul(Mpz::from(2)) * &a, a.pow(2))
        }
    }
}

pub fn _fib(x: IndexType) -> Mpz {
    let (a, b) = _no_parralel_fib(x);
    a + b
}

pub fn fib(x: IndexType) -> Mpz {
    if x < 3 {
        Mpz::from(1)
    } else {
        if x % 2 == 1 {
            let n = (x + 1) / 2;
            let (a, b) = join(|| fib(n).pow(2), || fib(n - 1).pow(2));
            a + b
        } else {
            let n = x / 2;
            let (a, b) = join(|| fib(n), || fib(n - 1));
            (b.mul(Mpz::from(2)) + &a) * a
        }
    }
}
