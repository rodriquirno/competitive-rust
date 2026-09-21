// 1037D. Valid BFS?

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

fn solve(n: usize, edges: &mut [Vec<usize>], seq: Vec<usize>, pos: Vec<usize>) -> bool {
    let mut vis: Vec<i8> = vec![-1; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut i: usize = 1;

    vis[0] = 0;
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        edges[u].sort_by_key(|&x| pos[x]); // force bfs to take seq path
        for &v in &edges[u] {
            if vis[v] == -1 {
                if seq[i] - 1 != v {
                    return false;
                }
                vis[v] = 0;
                q.push_back(v);
                i += 1;
            }
        }
        vis[u] = 1;
    }
    true
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut seq: Vec<usize> = vec![0; n];
    let mut pos: Vec<usize> = vec![0; n];

    for i in 0..(n - 1) {
        let e: Vec<usize> = sc.vec(2);
        edges[e[0] - 1].push(e[1] - 1);
        edges[e[1] - 1].push(e[0] - 1);
    }

    seq = sc.vec(n);

    for i in 0..n {
        pos[seq[i] - 1] = i;
    }

    let res: bool = solve(n, &mut edges, seq, pos);
    writeln!(out, "{}", if res { "Yes" } else { "No" }).unwrap();
}
