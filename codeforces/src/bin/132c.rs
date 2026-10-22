// 132C. Logo Turtle

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

const UND: i16 = i16::MAX;
const NINF: i16 = i16::MIN/ 2;

fn solve(i: usize, n: isize, d: usize, tf: &str, dp: &mut[Vec<Vec<i16>>]) -> i16 {
    if n < 0 { return NINF; }
    if i == tf.len() {
        if n % 2 == 0 { return 0; }
        else { return NINF; }
    }
    if dp[i][n as usize][d] != UND { return dp[i][n as usize][d]; }
    
    if tf.as_bytes()[i] == b'F' {
        dp[i][n as usize][d] = max(
            solve(i + 1, n - 1, 1 - d, tf, dp),    
            (solve(i + 1, n, d, tf, dp) + (2*d as i16 - 1)) // swap
        );
    } else {
        dp[i][n as usize][d] = max(
            (solve(i + 1, n - 1, d, tf, dp) + (2*d as i16 - 1)),
            (solve(i + 1, n, 1 - d, tf, dp))   // swap
        );
    }

    return dp[i][n as usize][d]
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let tf: String = sc.next();
    let n: isize = sc.next();

    let mut dp: Vec<Vec<Vec<i16>>> = vec![vec![vec![UND; 2]; n as usize + 1]; tf.len()];
    
    let l = solve(0, n, 0, &tf, &mut dp);
    let r = solve(0, n, 1, &tf, &mut dp);

    writeln!(out, "{}", max(l, r)).unwrap();
}

