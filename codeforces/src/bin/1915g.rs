// 1915G. Bicycles

#![allow(unused)]

use std::{collections::BinaryHeap, io::{self, Read, Write, stdin}};
use std::cmp::Reverse;

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

const INF: usize = usize::MAX;

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let mut t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let m: usize = sc.next();
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        for _ in 0..m {
            let u: usize = sc.next();
            let v: usize = sc.next();
            let w: usize = sc.next();
            adj[u - 1].push((v - 1, w));
            adj[v - 1].push((u - 1, w));
        }
        let s: Vec<usize> = sc.vec(n);
        let mut d: Vec<Vec<usize>> = vec![vec![INF; n]; n];
        let mut q = BinaryHeap::new();

        d[0][0] = 0;
        q.push(Reverse((0usize, 0usize, 0usize)));

        while let Some(Reverse((du, u, mut b))) = q.pop() { // lazy deletion
            if u == n - 1 { writeln!(out, "{du}").unwrap(); break; }
            if d[u][b] < du { continue; }
            b = if s[b] > s[u] { u } else { b };
                for &(v, w) in &adj[u] {
                    if d[v][b] > du + w*s[b] {
                        d[v][b] = du + w*s[b];
                        q.push(Reverse((d[v][b], v, b)));
                }
            }
        }
    }
}
