// 2259A. Moo Language School

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

    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn solve(n: usize, k: usize, nhoj: &Vec<u8>) -> u8 {
    let mut c: u8 = 0;
    for i in 0..(n / k) {
        if nhoj[i * k..(i + 1) * k].iter().all(|&b| b == 1) {
            c += 1;
        }
    }
    c
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();
        let bits: String = sc.next();
        let nhoj: Vec<u8> = bits.bytes().map(|b| b - b'0').collect();
        let mut res: u8 = solve(n, k, &nhoj);
        writeln!(out, "{}", res).unwrap();
    }
}
