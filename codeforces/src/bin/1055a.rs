// 1055A. Metro

#![allow(unused)]

use std::io::stdin;

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    arr
}

fn solve(n: usize, s: usize, a: Vec<usize>, b: Vec<usize>) -> bool {
    if (a[0] == 0) || (a[s] == 0 && b[s] == 0) {
        return false;
    }
    if a[s] == 1 {
        return true;
    }
    for i in s..n {
        if a[i] == 1 && b[i] == 1 {
            return true;
        }
    }
    false
}

pub fn main() {
    let first = take_vector();
    let n = first[0];
    let s = first[1] - 1;
    let a = take_vector();
    let b = take_vector();

    let res: bool = solve(n, s, a, b);
    println!("{}", if res { "YES" } else { "NO" });
}
