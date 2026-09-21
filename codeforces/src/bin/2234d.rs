// 2234D. XOR, Expression and Two Binary Numbers

#![allow(unused)]

use std::{
    collections::HashMap,
    io::{self, Read, Write, stdin},
};

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

fn solve(n: usize, k: usize, s: &[u8], z: &[u8]) -> u64 {
    // XOR
    let w: Vec<u8> = s.iter().zip(z).map(|(x, y)| x ^ y).collect();
    let c = 2u64.pow(k as u32); // or  c: u64 = 1u64 << k, accepts k as usize
    let s_xy = xy(n, s);
    let z_xy = xy(n, z);
    let w_xy = xy(n, &w);

    let mut sum: u64 = (s_xy + z_xy + w_xy) * (c + 1);

    if k % 2 == 0 {
        sum += s_xy + z_xy - 2 * w_xy;
    };
    sum / 3
}

fn xy(n: usize, a: &[u8]) -> u64 {
    let ones: u64 = a.iter().map(|&b| b as u64).sum();
    ((n as u64) - ones) * ones
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();
        let s: Vec<u8> = sc.next::<String>().bytes().map(|b| b - b'0').collect();
        let z: Vec<u8> = sc.next::<String>().bytes().map(|b| b - b'0').collect();
        let mut res: u64 = solve(n, k, &s, &z);
        writeln!(out, "{res}").unwrap();
    }
}
