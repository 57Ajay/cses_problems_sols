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

        if s1 + s2 > n {
            println!("NO");
            continue;
        }

        if s1 + s2 == 1 {
            println!("NO");
            continue;
        }

        println!("YES");

        let a: Vec<usize> = (1..=n).collect();
        let mut b = a.clone();

        let k = s1 + s2;
        if k > 0 {
            let mut shifted_part = vec![0; k];
            for i in 0..k {
                let original_idx = (i + k - s2) % k;
                shifted_part[i] = a[original_idx];
            }

            for i in 0..k {
                b[i] = shifted_part[i];
            }
        }

        for (i, x) in a.iter().enumerate() {
            print!("{}{}", x, if i == n - 1 { "" } else { " " });
        }
        println!();

        for (i, x) in b.iter().enumerate() {
            print!("{}{}", x, if i == n - 1 { "" } else { " " });
        }
        println!();
    }
}
