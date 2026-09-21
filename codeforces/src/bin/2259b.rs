// 2259B. Minus Two

#![allow(unused)]

use std::cmp::max;
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

fn solve(n: usize, a: &[usize]) -> usize {
    let even_1: usize = a.iter().filter(|&&x| x % 4 == 0).count();
    let even_2: usize = a.iter().filter(|&&x| x % 4 == 2).count();
    let odd = n - even_1 - even_2;
    odd.max(even_1).max(even_2)
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: Vec<usize> = sc.vec(n);
        let res: usize = solve(n, &a);
        writeln!(out, "{res}").unwrap();
    }
}
