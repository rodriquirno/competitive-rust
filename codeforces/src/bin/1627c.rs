// 1627C. Not Assigning

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
}

fn dfs(v: usize, adj: &[Vec<(usize, usize)>], p: &mut [usize], mut last: usize) {
    for &(u, i) in &adj[v] {
        if  p[i] == 0 {
            p[i] = last;
            if last == 2 { last = 5; } else { last = 2; }
            dfs(u, adj, p, last)
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut p: Vec<usize> = vec![0; n - 1];
        let mut d: Vec<usize> = vec![0; n];
        let mut posible = true;
        for i in 0..n-1 {
            let u = sc.next::<usize>() - 1;
            let v = sc.next::<usize>() - 1;
            adj[u].push((v, i));
            adj[v].push((u, i));
            d[u] += 1;
            d[v] += 1;
            if d[u] > 2 || d[v] > 2 {
                posible = false; }
        }
        if posible {
            dfs(0, &adj, &mut p, 2);
            for q in p {
                write!(out, "{} ", q).unwrap();
            }
            writeln!(out).unwrap();
        } else {
            writeln!(out, "{}", -1).unwrap();
        }
    }
}
