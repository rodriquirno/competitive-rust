// 2262A1. Floor of MEX (Easy Version)

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
    
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn solve(n: usize, a: &[usize]) -> Vec<usize>{
    let mut not_b = vec![false; n];
    for k in 1..=n {
        let ak = a[k - 1];
        let min = k * ak; 
        if min >= n { continue; }  
        let max = (min + k).min(n); 
        for v in min..max {      
            not_b[v] = true; 
        }
    }

    (0..n).filter(|&i| !not_b[i]).collect()
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let a: Vec<usize> = sc.vec(n);

        let res: Vec<usize> = solve(n, &a);
        writeln!(out, "{}", res.len());
        for x in res {
            write!(out, "{} ", x).unwrap();
        }
        writeln!(out).unwrap();
        out.flush().unwrap();
    }
}
