// 2263A. Min Max Game

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
    
    fn vec<T: std::str::FromStr>(&mut self, n: u8) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn solve(n: u8, a: &[u8]) -> String {
    let mut ones: u8 = a.into_iter().sum();
    let mut zeros = n - ones;
    if ones >= zeros {
        return String::from("Bessie");
    }
    String::from("Elsie")
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: u16 = sc.next();
    for _ in 0..t {
        let n: u8 = sc.next();
        let a: Vec<u8> = sc.vec(n);

        let res: String = solve(n, &a);

        writeln!(out, "{}", res).unwrap();
    }
}