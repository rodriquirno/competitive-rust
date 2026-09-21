// 2264A. Rumb Needs a Hand

#![allow(unused)]

use std::io::{self, Read, Write, BufWriter};

struct Scanner {
    buf: Vec<String>,
    idx: usize,       
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new(); 
        io::stdin().read_to_string(&mut s).unwrap(); 
        let buf: Vec<String> = s.split_whitespace().map(|t| t.to_string()).collect(); 
        Scanner { buf, idx: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let t = &self.buf[self.idx];
        self.idx += 1;
        t.parse().ok().expect("parse error")
    }

    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn solve(n: usize, p: &[usize]) -> bool {
    let a: Vec<usize> = (0..n).filter(|&i| p[i] != i + 1).collect();
    let m = a.len();
    let ok = (0..m).all(|j| p[a[j]] == a[m - 1 - j] + 1);
    ok
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: u16 = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let p: Vec<usize> = sc.vec(n);

        let res = solve(n, &p);
        writeln!(out, "{}", if res { "YES" } else { "NO" }).unwrap();
    }
}