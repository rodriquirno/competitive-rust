// 1245D. Shichikuji and Power Grid

#![allow(unused)]

use std::{collections::BinaryHeap, io::{self, Read, Write, stdin}};

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

const INF: i64 = i64::MAX / 4;
const NIL: usize = usize::MAX;


fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let mut xs: Vec<i64> = Vec::new();
    let mut ys: Vec<i64> = Vec::new();
    for i in 0..n {
        xs.push(sc.next());
        ys.push(sc.next());
    }
    let c: Vec<i64> = sc.vec(n);
    let k: Vec<i64> = sc.vec(n);

    let w = |i: usize, j: usize| -> i64 {
        if i == j { INF }
        else if j == n { c[i] }
        else if i == n { c[j] }
        else { (k[i] + k[j]) * ((xs[i] - xs[j]).abs() + (ys[i] - ys[j]).abs())}
    };

    // Prim
    let mut d: Vec<i64> = vec![INF; n + 1];
    let mut p: Vec<usize> = vec![NIL; n + 1];
    let mut t: Vec<bool> = vec![false; n + 1];
    
    d[n] = 0; // s = n
    for _ in 0..n {
        let mut u = NIL;
        for v in 0..=n {
            if !t[v] && (u == NIL || d[v] < d[u]) {
                u = v;
            }
        }
        t[u] = true;
        for v in 0..=n {
            if !t[v] && w(u, v) < d[v] {
                p[v] = u;
                d[v] = w(u, v);
            }
        }
    }

    // Answer
    let mut r: Vec<usize> = Vec::new();
    let mut e: Vec<(usize, usize)> = Vec::new();
    let mut cost = 0i64;

    for v in 0..n {
        if p[v] == n { r.push(v); }
        else {
            e.push((v, p[v]));
        }
        cost += d[v];
    }
    
    writeln!(out, "{cost}").unwrap();
    writeln!(out, "{}", &r.len()).unwrap();
    for v in r { write!(out, "{} ", v+1).unwrap(); }
    writeln!(out, "").unwrap();
    writeln!(out, "{}", &e.len()).unwrap();
    for (u, v) in &e {
        writeln!(out, "{} {} ", u + 1, v + 1).unwrap();
    }
}
