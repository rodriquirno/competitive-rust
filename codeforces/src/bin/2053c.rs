// 2053C. Bewitching Stargazer

#![allow(unused)]

use std::collections::HashMap;
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

fn solve(l: usize, r: usize, k: usize, memo: &mut HashMap<usize, (u64, u64)>) -> (u64, u64) {
    if l > r {
        return (0, 0);
    }
    let s: usize = r - l + 1;
    let d = (l - 1) as u64;

    if s < k {
        return (0, 0);
    }
    if let Some(&(cc, sum_base)) = memo.get(&s) {
        return (cc, sum_base + d * cc);
    }

    let m = (l + r) / 2;
    let (c, sum): (u64, u64) = if s % 2 == 0 {
        let (cl, suml) = solve(l, m, k, memo);
        let (cr, sumr) = solve(m + 1, r, k, memo);
        (cl + cr, suml + sumr)
    } else {
        let (cl, suml) = solve(l, m - 1, k, memo);
        let (cr, sumr) = solve(m + 1, r, k, memo);
        (cl + cr + 1, suml + sumr + m as u64)
    };
    memo.insert(s, (c, sum - d * c));
    (c, sum)
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();
        let mut memo = HashMap::new();
        let mut res: u64 = solve(1, n, k, &mut memo).1;
        writeln!(out, "{}", res).unwrap();
    }
}
