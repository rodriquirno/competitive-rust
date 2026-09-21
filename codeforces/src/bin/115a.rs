// 115A. Party

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
}

fn solve(n: usize, parent: Vec<Option<usize>>, children: Vec<Vec<usize>>) -> i64 {
    let mut h = 0_i64; // max_height

    fn dfs(v: usize, depth: i64, children: &Vec<Vec<usize>>, h: &mut i64) {
        if depth > *h {
            *h = depth;
        }
        for &u in &children[v] {
            dfs(u, depth + 1, children, h)
        }
    }

    for i in 0..n {
        if parent[i].is_none() {
            dfs(i, 1, &children, &mut h)
        }
    }
    h
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();

    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];

    for i in 0..n {
        let p: i64 = sc.next();
        if p != -1 {
            let par = (p - 1) as usize;
            parent[i] = Some(par);
            children[par].push(i);
        }
    }

    let res: i64 = solve(n, parent, children);
    writeln!(out, "{}", res).unwrap();
}
