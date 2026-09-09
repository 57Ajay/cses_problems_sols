use std::{
    io::{Read, stdin},
    println,
};

pub fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut t = Vec::<(u32, i32)>::with_capacity(2 * n);

    loop {
        let a = match iter.next().and_then(|v| v.parse::<u32>().ok()) {
            Some(n) => n,
            None => break,
        };
        let d = match iter.next().and_then(|v| v.parse::<u32>().ok()) {
            Some(n) => n,
            None => break,
        };

        t.push((a, 1));
        t.push((d, -1));
    }

    t.sort_unstable();

    let mut max = 0;
    let mut c = 0;

    for v in t {
        c += v.1;
        max = max.max(c);
    }

    println!("{}", max);
}
