// 2245B. Delete and Concatenate

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

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());


    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let c: i64 = sc.next();
        let mut a: Vec<i64> = sc.vec(n);

        a.sort();
        let mut sum = a.iter().sum::<i64>() - c * n as i64;

        for i in 0..n/2 {
            if a[i] < c { sum += c - a[i]; }
            else { break; }
        }
        
        writeln!(out, "{sum}").unwrap();
    }
}
