// 1278B. A and B

#![allow(unused)]

use std::{io::{self, Read, Write, stdin}, iter::Sum};

struct Scanner {
    buf: Vec<String>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        stdin().read_to_string(&mut s).unwrap();
        let buf: Vec<String> = s.split_whitespace().map(|t| t.to_string()).collect();
        Scanner { buf, idx: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let t = &self.buf[self.idx];
        self.idx += 1;
        t.parse().ok().expect("error de parseo")
    }

    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: u8 = sc.next();
    for _ in 0..t {
        let a: usize = sc.next();
        let b: usize = sc.next();

        if a == b {
            writeln!(out, "0").unwrap();
        } else {
            let mut n: isize = 1;
            let mut s: isize = 0;
            loop {
                s += n;
                let d = if a > b { a - b } else { b - a };
                if (s - d as isize) % 2 == 0 && d as isize <= s {
                    break;
                }
                n += 1;
            }
            writeln!(out, "{n}").unwrap();
        }
    }
}

