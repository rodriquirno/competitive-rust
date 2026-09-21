// 782B. The Meeting Place Cannot Be Changed

#![allow(unused)]

use std::io::{self, Read, Write, BufWriter};
use std::cmp::{max, min};

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

fn solve(n: usize, x: &[usize], v: &[usize]) -> f64 {
    let t_min = 0f64;
    let (min, max) = x.iter().fold((usize::MAX, 0usize), |(lo, hi), &y| {(lo.min(y), hi.max(y))});
    let t_max = (max - min) as f64;
    find_min_t(x, v, t_min, t_max)
}

fn find_min_t(x: &[usize], v: &[usize], lo: f64, hi: f64) -> f64 {
    if hi - lo < 1e-9 * hi.max(1.0) {
        return hi;
    }
    let t = (lo + hi) / 2.0;
    if posible(x, v, t) {
        return find_min_t(x, v, lo, t);
    }
    find_min_t(x, v, t, hi)
}

fn posible(x: &[usize], v: &[usize], t: f64) -> bool {
    let (l_max, r_min) = x.iter()
        .zip(v)
        .fold((f64::NEG_INFINITY, f64::INFINITY), |(l, r), (&xi, &vi)| {
            let (xi, vi) = (xi as f64, vi as f64);
            (l.max(xi - t * vi), r.min(xi + t * vi))
        });
    l_max <= r_min
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let n: usize = sc.next();
    let x: Vec<usize> = sc.vec(n);
    let v: Vec<usize> = sc.vec(n);

    let res = solve(n, &x, &v);
    writeln!(out, "{res}").unwrap();
}