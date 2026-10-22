// 2267A. Turn Into a Palindrome

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

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let c: u8 = sc.next::<char>() as u8;
        let mut s: String = sc.next();
        let s: &mut [u8] = unsafe { s.as_bytes_mut() };
        let mut i = 0;
        let mut j = n - 1;
        let mut a = 0;
        while i < j {
            if s[i] != s[j] {
                if s[i] == c { s[j] = c; a += 1; }
                else if s[j] == c { s[i] = c; a += 1;}
                else { s[i] = c; s[j] = c; a += 2}
            }
            i += 1; j -= 1;
        }
        writeln!(out, "{a}").unwrap();
    }
}

