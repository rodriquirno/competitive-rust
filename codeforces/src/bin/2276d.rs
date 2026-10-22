// 2276D. Caveman

#![allow(unused)]

use std::{collections::HashMap, io::{self, stdin, Read, Write}};
use std::collections::VecDeque;

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

fn solve(n: usize, adj: &[Vec<usize>]) -> Vec<(usize, usize, usize)> {
    let mut c = Vec::new();
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut vis: Vec<bool> = vec![false ; n];

    for r in 0..n {
        if vis[r] {
            continue;
        }
        let mut s = 0usize;
        let mut ds = 0usize;
        let mut md = 0usize;

        vis[r] = true;
        q.push_back(r); // BFS
        while let Some(u) = q.pop_front() { 
            s += 1;
            ds += adj[u].len();
            md = md.max(adj[u].len());
            for &v in &adj[u] {
                if !vis[v] {
                    vis[v] = true;
                    q.push_back(v);
                }
            }
        }
        c.push((s, ds / 2, md));
    }
    c
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    
    let t: usize = sc.next();
    for i in 0..t {  
        let n: usize = sc.next();
        let m: usize = sc.next();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for _ in 0..m {
            let u: usize = sc.next();
            let v: usize = sc.next();
            adj[u - 1].push(v - 1); 
            adj[v - 1].push(u - 1);
        }

        let c = solve(n, &adj);
        let mut res = -1;
        if c.iter().any(|&(s, _, _)| s == 1) {
            writeln!(out, "Brain Melt").unwrap();
        } else if c.len() == 1 {
            let (s, e, md) = c[0];
            if e == s - 1 && md == s - 1 {
                writeln!(out, "2").unwrap();
            } else {
                writeln!(out, "Many").unwrap();
            }
        } else {
            writeln!(out, "Many").unwrap();
        }
    }
}