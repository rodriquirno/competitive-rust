// 2259C. 101

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

fn solve(n: usize, a: &mut Vec<i8>) -> Vec<i8> {
    let mut l = 0;
    while l < n && a[l] == 0 {
        l += 1;
    }
    let mut r = n;
    while r > 0 && a[r - 1] == 0 {
        r -= 1;
    }
    for x in a.iter_mut() {
        if *x == -1 {
            *x = 0;
        }
    }
    if l < n {
        a[l] = 1;
        a[r - 1] = 1;
    }
    a.to_vec()
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut a: Vec<i8> = sc.vec(n);
        let res: Vec<i8> = solve(n, &mut a);
        for (i, x) in res.iter().enumerate() {
            if i > 0 {
                write!(out, " ").unwrap();
            }
            write!(out, "{}", x).unwrap();
        }
        writeln!(out).unwrap();
    }
}
