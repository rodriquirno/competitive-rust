// 2266B. Three Piles

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
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for t in 0..t {
        let a: isize = sc.next();
        let b: isize = sc.next();
        let c: isize = sc.next();
        
        let mut s = (a - b).abs();
        if a >= b || c >= 2 * s {
            s = a + c - b;
        }
        writeln!(out, "{s}").unwrap();
    }
}

