// 2276A. Perfect Sandwiches

#![allow(unused)]

use std::{io::{self, Read, Write, stdin}, iter::Sum};
use std::cmp::max;

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

    let t: usize = sc.next();
    for _ in 0..t {
        let mut a: u8 = sc.next();
        let mut b: u8 = sc.next();
        if b >= a {
            writeln!(out, "-1").unwrap();
        } else if b <=  a-b {
            writeln!(out, "{b}").unwrap();
        } else {
            writeln!(out, "{}", a-b).unwrap();
        }
    }
}

