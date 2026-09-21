// 2263B. Min Matrices

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

fn solve(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k < n || k == 2 * n {
        return Vec::new();
    }
    let mut a: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        a.push(((i * n + 1)..=((i + 1) * n)).collect()); 
    }
    for i in 1..(2 * n - k) {
        let temp = a[0][i];
        a[0][i] = a[i][i];
        a[i][i] = temp;
    }
    a
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: u16 = sc.next();
    for i in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();

        let res: Vec<Vec<usize>> = solve(n, k);
        if res.is_empty() {
            writeln!(out, "-1").unwrap();
        }
        for row in &res {
            for x in row {
                write!(out, "{} ", x).unwrap();
            }
            writeln!(out).unwrap();
        }
    }
}
