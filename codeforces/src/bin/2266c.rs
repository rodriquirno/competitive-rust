// 2266C. AND, OR, Sort!

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
    for t in 0..t {
        let n: usize = sc.next();
        let s: String = sc.next();
        let s = s.as_bytes();
        let mut a = n;
        let mut o = 0;
        let mut z = s.iter().filter(|&&c| c == b'0').count();
        if s[0] == b'1' {
            a = z;
        } else {
            for i in 0..n {
                if s[i] == b'1' { o += 1; }
                else { z -= 1; }
                a = a.min(o + z);
            }
        }
        writeln!(out, "{a}").unwrap();
    }
}

