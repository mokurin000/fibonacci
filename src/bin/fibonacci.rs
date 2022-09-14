use clap::Parser;
use fibonacci::fib;
use fibonacci::IndexType;

fn main() {
    let Args {
        n,
        r: range,
        print_result,
    } = Args::parse();

    match (n, range) {
        (_, Some(pat)) => {
            for r in pat.split(",").map(|s| {
                if s.is_empty() {
                    return 1..=0;
                }

                if s.contains("..=") {
                    let mut split = s.split("..=");
                    let l = split.next().unwrap().parse().unwrap();
                    let r = split.next().unwrap().parse().unwrap();
                    l..=r
                } else {
                    let n: IndexType = s.parse().unwrap();
                    n..=n
                }
            }) {
                for i in r {
                    if print_result {
                        println!("{i}: {}", fib(i));
                    }
                }
            }
        }
        (Some(n), _) => {
            let result = fib(n);
            if print_result {
                println!("{result}");
            }
        }
        (_, _) => eprintln!("you should provide either n or r to use me!\n"),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// specify to calculate Fib[n].
    /// n is stored in u64.
    #[clap(short, value_parser)]
    n: Option<IndexType>,

    /// specify to calculate a range of fibonacci's.
    /// this will override `n`.
    ///
    /// Note: indices are currently stored in u64.
    ///
    /// format: index1,index2,start..=end
    #[clap(short, long = "range", value_parser)]
    r: Option<String>,

    /// whether should we print the result or only perform calculation.
    #[clap(short = 'p', long = "print", value_parser)]
    print_result: bool,
}
