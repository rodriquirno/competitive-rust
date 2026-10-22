// 2275B. Did Not Go to Print

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

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let mut n: usize = sc.next();
        let s: Vec<usize> = sc.next::<String>().bytes().map(|b| (b - b'0') as usize).collect();
        let mut q = Vec::new();
        let mut l = vec![false; n];
        for (idx, i) in s.iter().enumerate() {
            match i {
                1 => { q.push(idx) },
                2 => { 
                    if let Some(j) = q.pop() {
                        l[j] = true;
                    } else {
                        l[idx] = true;
                    }
                    n -= 1;
                },
                3 => { l[idx] = true; n -= 1; },
                _ => (),
            }
        }

        writeln!(out, "{n}").unwrap();
        if n != 0 {
            for i in 0..l.len() {
                if !l[i] { write!(out, "{} ", i + 1).unwrap(); }
            }
            writeln!(out).unwrap();
        } else {
            writeln!(out, "").unwrap();
        }
    }
}

