// 1419C. Killjoy

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
        let n: usize = sc.next();
        let x: isize = sc.next();
        let mut a: Vec<isize> = sc.vec(n);
        let s: isize = a.iter().sum();
        let min = a.iter().max().unwrap();
        let max = a.iter().min().unwrap();
        if *min == x && *max == x {
            writeln!(out, "0").unwrap();
        } else if s == n as isize * x || a.contains(&x) {
            writeln!(out, "1").unwrap();
        } else {
            writeln!(out, "2").unwrap();
        }
    }
}

