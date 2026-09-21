// 664D. Graph Coloring

#![allow(unused)]

use std::{collections::HashMap, io::{self, stdin, Read, Write}};
use std::collections::VecDeque;

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

fn solve(n: usize, edges: &[Vec<(usize, u8)>],) -> Option<Vec<usize>> {
    let mut best: Option<Vec<usize>> = None;
    
    for color in [b'R', b'B'] {
        let mut bit: Vec<i8> = vec![-1; n];
        let mut temp: Option<Vec<usize>> = Some(Vec::new());
        let mut q: VecDeque<usize> = VecDeque::new();
        
        // BFS
        'bfs_loop: for i in 0..n {
            let mut comp: VecDeque<usize> = VecDeque::new();
            let mut ones = 0usize;
            if bit[i] != -1 { continue; }
            bit[i] = 0;
            q.push_back(i);
            comp.push_back(i);
            while let Some(u) = q.pop_front() {
                for &(v, c) in &edges[u] {
                    if bit[v] == -1 {
                        bit[v] = if c == color { bit[u] } else { 1 - bit[u] };
                        if bit[v] == 1 {
                            ones += 1; 
                        }
                        comp.push_back(v);
                        q.push_back(v);
                    } else {
                        let expected = if c == color { bit[u] } else { 1 - bit[u] };
                        if bit[v] != expected {
                            temp = None;
                            break 'bfs_loop;
                        }
                    }
                }
            }

            let take_ones = ones * 2 <= comp.len();
            if let Some(temp) = temp.as_mut() {
                for &w in &comp {
                    let is_one = bit[w] == 1;
                    if is_one == take_ones {
                        temp.push(w)
                    }
                }
            }
        }

        if !temp.is_none() {
            match best {
                Some(b) => { best = if temp.as_ref().unwrap().len() < b.len() { temp } else { Some(b)} },
                None    => { best = temp}
            }
        } 
    }   
    best
}

fn main() {
    let mut sc = Scanner::new();
    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    
    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut edges: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let c = sc.next::<String>().as_bytes()[0]; // b'R' or b'B'
        edges[u - 1].push((v - 1, c));
        edges[v - 1].push((u - 1, c));
    }

    let mut res: Option<Vec<usize>> = solve(n, &edges);
    if res.is_none() {
        write!(out, "{}", -1).unwrap();
    } else {
        writeln!(out, "{}", res.as_ref().unwrap().len());
        for (i, x) in res.unwrap().iter().enumerate() {
            if i > 0 { write!(out, " ").unwrap(); }
            write!(out, "{}", x + 1).unwrap();
        }
    }
    writeln!(out).unwrap();
}