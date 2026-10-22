// 1207C. Gas Pipeline

#![allow(unused)]

use std::io::{self, Read, Write, stdin};
use std::cmp::min;

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

const INF: usize = usize::MAX / 2;

fn solve(s: &[u8], a: usize, b: usize, dp: &mut [Vec<Option<usize>>], mut i: usize, mut h: usize) -> usize {
    if let Some(x) = dp[h][i] { return x; }
    if s[i] == b'1' {
        if h == 0 { dp[h][i] = Some(INF); }
        else { 
            dp[h][i] = Some((h+1)*b + a + solve(s, a, b, dp, i + 1, h ))
        }
    }
    if s[i] == b'0' {
        if i == s.len() - 1 { return (h + 1)* a + (h + 2)*b; }
        dp[h][i] = Some(min(
            (h+1)*b + a + solve(s, a, b, dp, i + 1, h),       // same h
            (h+1)*b + 2*a + solve(s, a, b, dp, i + 1, h ^ 1))) // change h
    }

    return dp[h][i].unwrap()
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: u8 = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: usize = sc.next();
        let b: usize = sc.next();
        let s: String = sc.next();
        let s = s.as_bytes(); // &[u8]
        let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; n]; 2];
        
        let res = solve(s, a, b, &mut dp, 0, 0);
        
        writeln!(out, "{res}").unwrap();
    }
}

