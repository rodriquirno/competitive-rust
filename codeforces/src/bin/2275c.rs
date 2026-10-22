// 2275C. Unrequited Love

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
        let mut n: usize = sc.next();
        let a: Vec<i32> = sc.vec(n);
        let s: Vec<i32> = (0..n-4).map(|x| a[x] + a[x + 2] - a[x + 4]).collect();
        let mut p: HashMap<i32, u64> = HashMap::new();
        let mut c = 0;

        for &v in &s {
            let e = p.entry(v).or_insert(0);
            c += *e;
            *e += 1;
        }
        for x in 0..n-4 {
            if x + 6 < n && s[x] == s[x + 2] { c -= 1; }
            if x + 8 < n && s[x] == s[x + 4] { c -= 1; }
        }

        writeln!(out, "{c}").unwrap();
    }
}

