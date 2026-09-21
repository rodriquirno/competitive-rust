// 580C. Kefa and Park

#![allow(unused)]

use std::collections::VecDeque;
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

fn solve(n: usize, m: usize, a: &[usize], edges: &[Vec<usize>]) -> usize {
    let mut vis = vec![false; n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::with_capacity(n);
    let mut res: usize = 0;

    vis[0] = true;
    q.push_back((0, a[0]));
    while let Some((u, c)) = q.pop_front() {
        if c > m {
            continue;
        }
        let mut leaf = true;
        for &v in &edges[u] {
            if !vis[v] {
                leaf = false;
                vis[v] = true;
                let new_c = if a[v] == 1 { c + 1 } else { 0 };
                q.push_back((v, new_c));
            }
        }
        if leaf {
            res += 1
        }
    }
    res
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let m: usize = sc.next();
    let a: Vec<usize> = sc.vec(n);
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        let e: Vec<usize> = sc.vec(2);
        edges[e[0] - 1].push(e[1] - 1);
        edges[e[1] - 1].push(e[0] - 1);
    }

    let res: usize = solve(n, m, &a, &edges);
    writeln!(out, "{res}").unwrap();
}
