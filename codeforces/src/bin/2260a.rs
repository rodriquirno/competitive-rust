// 2260A. Monocarp's Contest

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

fn solve(n: usize, a: &mut[u8]) -> i8 {
    let first = a[0] == 0;
    let last  = a[n-1] == 0;
    if first && last { return 0 }
    let mut zeros = 0u8;
    for i in 1..n-1 {
        zeros += if a[i] == 0 { 1 } else { 0 };
        if (first || last) && zeros == 1 { return 1 }
        if zeros == 2 { return 2 }
    }
    -1
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut a: Vec<u8> = sc.vec(n);

        let res: i8 = solve(n, &mut a);

        writeln!(out, "{}", res).unwrap();
    }
}