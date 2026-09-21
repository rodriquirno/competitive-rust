// 1559C. Mocha and Hiking

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

fn solve(n: usize, a: &[usize]) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(n + 1);

    if a[n - 1] == 0 {
        res = (0..=n).collect();
    } else if a[0] == 1 {
        res.push(n);
        res.extend(0..n);
    } else {
        for i in 0..n {
            if a[i] == 0 && a[i + 1] == 1 {
                res = (0..=i).collect();
                res.push(n);
                res.extend(i + 1..=n - 1);
            }
        }
    }
    res
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: Vec<usize> = sc.vec(n);
        let mut res: Vec<usize> = solve(n, &a);

        for (i, x) in res.iter().enumerate() {
            if i > 0 {
                write!(out, " ").unwrap();
            }
            write!(out, "{}", x + 1).unwrap();
        }
        writeln!(out).unwrap();
    }
}
