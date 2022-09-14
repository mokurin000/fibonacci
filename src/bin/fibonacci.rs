use fibonacci::fib;
use gmp::mpz::Mpz;

use clap::Parser;

fn main() {
    let Args { n, r: range } = Args::parse();

    match (n, range) {
        (_, Some(pat)) => {
            for r in pat.split(",").map(|s| {
                if s.is_empty() {return 1..=0}

                if s.contains("..=") {
                    let mut split = s.split("..=");
                    let l = split.next().unwrap().parse().unwrap();
                    let r = split.next().unwrap().parse().unwrap();
                    l..=r
                } else {
                    let n: u64 = s.parse().unwrap();
                    n..=n
                }
            }) {
                for i in r {
                    println!("{i}: {}", fib(i));
                }
            }
        }
        (Some(n), _) => println!("{}", fib(n)),
        (_, _) => eprintln!("you should provide either n or r to use me!\n")
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// specify to calculate Fib[n].
    /// n is stored in Mpz.
    #[clap(short, value_parser)]
    n: Option<Mpz>,

    /// specify to calculate a range of fibonacci's.
    /// this will override `n`.
    ///
    /// Note: indices are currently stored in u64.
    ///
    /// format: index1,index2,start..=end
    #[clap(short, long = "range", value_parser)]
    r: Option<String>,
}
