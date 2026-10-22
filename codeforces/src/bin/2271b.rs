// 2271B. MEX Game

#![allow(unused)]

use std::io::{self, Read, Write, stdin};
use std::collections::HashMap;

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
        let n: usize = sc.next();
        let k: usize = sc.next();
        let mut a: HashMap<usize, usize> = HashMap::new();

        for _ in 0..n {
            let x: usize = sc.next();
            *a.entry(x).or_insert(0) += 1;
        }

        let mut aa = false;
        for v in 0..=n {
            let c = a.get(&v).copied().unwrap_or(0);
            if c < 2*k {
                aa = c == 2 * k - 1;
                break;
            }
        }
        writeln!(out, "{}", if aa { "YES" } else { "NO" }).unwrap();
    }
}

