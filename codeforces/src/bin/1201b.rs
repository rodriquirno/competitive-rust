// 1201B. Zero Array

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

    let n: usize = sc.next();
    let mut a: Vec<usize> = sc.vec(n);
    a.sort();
    let s: usize = a.iter().sum();
    if s % 2 != 0 {
        writeln!(out, "NO").unwrap();
    } else if s - a[n-1] < a[n-1] {
        writeln!(out, "NO").unwrap();
    } else {
        writeln!(out, "YES").unwrap();
    }
}

