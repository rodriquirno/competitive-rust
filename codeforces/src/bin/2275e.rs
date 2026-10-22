// 2275E. Repentance Is Already on the Way

#![allow(unused)]

use std::{io::{self, Read, Write, stdin}, iter::Sum};
use std::cmp::max;

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

const UND: u32 = u32::MAX;

fn f(i: usize, a: &[u32], b: &[u32], dp: &mut [u32], z: &[u32]) -> u32 {
    if i == a.len() - 1 { return w(a[i], b[i]) as u32 }
    if dp[i] != UND { return dp[i] }

    dp[i] = max(
        w(a[i], b[i]) + w(b[i], a[i + 1]) + f(i + 1, a, b, dp, z),
        z[i]
    );
    dp[i]
}

fn w(a: u32, b: u32) -> u32 {
    if a == b { return 2; }
    return 1;
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: Vec<u32> = sc.vec(n);
        let b: Vec<u32> = sc.vec(n);
        let mut vis: Vec<u8> = vec![0; 2*n];
        let mut dp: Vec<u32> = vec![UND; n];
        let mut z: Vec<u32> = vec![0; n];
        z[n - 1] = w(a[n - 1], b[n - 1]);
        for j in (0..n-1).rev() {
            z[j] = z[j + 1] + w(a[j], b[j + 1]) + w(b[j], a[j + 1])
        }

        let res: u32 = f(0, &a, &b, &mut dp, &z); 
        writeln!(out, "{res}").unwrap();
    }
}

