# fibonacci

simple concurrent & blazingly fast high precision fibonacci calculator.

thanks to rayon's `join` and `rust-gmp`'s thread safe gmp wrapper.

## Usage

```bash
$ fibonacci -r 1..=3,5
1: 1
2: 1
3: 2
5: 5
$ fibonacci -n 114
298611126818977066918552
$ fibonacci --help
...
```

## TODO

- [ ] graceful range parse by nom
- [ ] memorize optimization for range caculate
