// 698A. Vacations

#![allow(unused)]

use std::io::{self, Read, Write, stdin};

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

fn solve(a: &[u8], mut i: usize, mut c: usize, mut l: u8) -> usize {
    if i == a.len() { return c; }
    match (a[i], l) {
        (0, _) => solve(a, i + 1, c + 1, 0),
        (1, 1) | (2, 2) => solve(a, i + 1, c + 1, 0),
        (1, _) => solve(a, i + 1, c, 1),
        (2, _) => solve(a, i + 1, c, 2),
        (3, 1) => solve(a, i + 1, c, 2),
        (3, 2) => solve(a, i + 1, c, 1),
        (3, _) => solve(a, i + 1, c, 3),
        _ => unreachable!(),
    }
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let a: Vec<u8> = sc.vec(n);
    
    let res = solve(&a, 0, 0, 3);
    
    writeln!(out, "{res}").unwrap();
}

