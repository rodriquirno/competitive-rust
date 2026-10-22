// 721C. Journey

#![allow(unused)]

use std::io::{self, Read, Write, stdin};

#[derive(Debug)]
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

const UD: u32 = u32::MAX;
const INF: u32 = u32::MAX - 1;
const NONE: u16 = u16::MAX;

fn solve(
    adj: &[Vec<(usize, u64)>], 
    dp: &mut [Vec<u32>],
    nxt: &mut [Vec<u16>],
    t_max: u64,
    v: usize,
    c: usize
) -> u32 {
    if dp[v][c] != UD { return dp[v][c]; }
    if v == adj.len() - 1 { return if c == 1 { 0 } else { INF }; }
    if c == 1 { return INF; }

    let mut best = INF;
    let mut nx = NONE;
    for &(w, t) in &adj[v] {
        let r = solve(adj, dp, nxt, t_max, w, c - 1);
        if r == INF { continue; }
        let total = r as u64 + t;
        if total <= t_max && (total as u32) < best {
            best = total as u32;
            nx = w as u16;
        }
    }
    dp[v][c] = best;
    nxt[v][c] = nx;
    best
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let n: usize = sc.next();
    let m: usize = sc.next();
    let t: u64 = sc.next();
    let mut adj: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n];
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let t: u64 = sc.next();
        adj[u - 1].push((v - 1, t));
    }
    let mut dp: Vec<Vec<u32>> = vec![vec![UD; n + 1]; n];
    let mut nxt: Vec<Vec<u16>> = vec![vec![NONE; n + 1]; n];

    let mut k = 0;
    for c in (2..=n).rev() {
        if solve(&adj, &mut dp, &mut nxt, t, 0, c) != INF {
            k = c;
            break;
        };
    }

    let mut res = vec![0];
    let mut v = 0;
    while v != n - 1 {
        v = nxt[v][k] as usize;
        k -= 1;
        res.push(v);
    }

    writeln!(out, "{}", res.len()); 
    for v in res {
        write!(out, "{} ", v + 1).unwrap();
    }
}

