// 510C. Fox and Names

#![allow(unused)]

use std::collections::VecDeque;
use std::io::{self, Read, Write, stdin};

use crate::Constrain::{Impossible, Nothing};

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

enum Constrain {
    Edge(usize, usize),
    Nothing,
    Impossible,
}
use Constrain::*;

fn solve(n: usize, ws: &[String]) -> String {
    let mut res = String::with_capacity(26);
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); 26];

    for pair in ws.windows(2) {
        match compare(&pair) {
            Edge(x, y) => edges[x].push(y),
            Nothing => (),
            Impossible => return String::from("Impossible"),
        }
    }

    let mut l = topo_sort(&edges);
    for b in &mut l {
        *b = b.wrapping_add(b'a') as u8;
    }
    if l.len() < 26 {
        return String::from("Impossible");
    }
    String::from_utf8(l).unwrap()
}

fn compare(pair: &[String]) -> Constrain {
    let a = pair[0].as_bytes();
    let b = pair[1].as_bytes();

    for x in a.iter().zip(b.iter()) {
        if x.0 != x.1 {
            return Edge(((*x.0 - b'a') as usize), ((*x.1 - b'a') as usize));
        }
    }

    if a.len() <= b.len() {
        return Nothing;
    }

    Impossible
}

fn topo_sort(edges: &[Vec<usize>]) -> Vec<u8> {
    let mut l: Vec<u8> = Vec::new();
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut d_in: Vec<usize> = vec![0usize; 26];

    for vs in edges {
        for &v in vs {
            d_in[v] += 1;
        }
    }

    for v in 0..26 {
        if d_in[v] == 0 {
            q.push_back(v);
        }
    }

    while let Some(u) = q.pop_front() {
        l.push(u as u8);
        for &v in &edges[u] {
            d_in[v] -= 1;
            if d_in[v] == 0 {
                q.push_back(v);
            }
        }
    }
    l
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let ws: Vec<String> = (0..n).map(|_| sc.next()).collect();

    let res: String = solve(n, &ws);
    writeln!(out, "{}", res).unwrap();
}
