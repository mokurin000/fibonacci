use rayon::join;

fn fib(x: u64) -> u64 {
    if x < 3 {
        1 as u64
    } else {
        if x % 2 == 1 {
            let n = (x + 1) / 2;
            let (a, b) = join(|| fib(n).pow(2), || fib(n - 1).pow(2));
            a + b
        } else {
            let n = x / 2;
            let (a, b) = join(|| fib(n), || fib(n - 1));
            (2 * b + a) * a
        }
    }
}

fn main() {
    let times = std::env::args().nth(1).unwrap().parse().unwrap();

    for i in 1..=times {
        let result = fib(i);
        println!("{i:>4}: {result}");
    }
}
