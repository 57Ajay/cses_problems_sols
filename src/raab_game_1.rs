use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = v[0];
        let s1 = v[1];
        let s2 = v[2];

        // impossible cases
        if s1 + s2 > n || (n % 2 == 1 && (s1 == 0 || s2 == 0)) {
            println!("NO");
            continue;
        }

        if 2 * (s1 + s2) > n {
            println!("NO");
            continue;
        }

        println!("YES");

        let a: Vec<usize> = (1..=n).collect();
        let mut b = vec![0; n];

        let mut idx = 0;

        // playar 1 wins
        for _ in 0..s1 {
            b[idx] = a[idx + 1];
            b[idx + 1] = a[idx];
            idx += 2;
        }

        // player 2 wins
        for _ in 0..s2 {
            b[idx] = a[idx];
            b[idx + 1] = a[idx + 1];
            idx += 2;
        }

        // draws
        for i in idx..n {
            b[i] = a[i];
        }

        for x in &a {
            print!("{} ", x);
        }
        println!();

        for x in &b {
            print!("{} ", x);
        }
        println!();
    }
}

