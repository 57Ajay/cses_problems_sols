use std::{
    collections::BTreeMap,
    io::{BufWriter, Read, StdoutLock, Write, stdin, stdout},
    writeln,
};

pub fn main() {
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    let stdin = stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut btmap = BTreeMap::<usize, usize>::new();

    for _ in 0..n {
        let key = iter.next().unwrap().parse::<usize>().unwrap();
        *btmap.entry(key).or_default() += 1;
    }

    let mut t = Vec::<usize>::with_capacity(m);

    while let Some(v) = iter.next() {
        let val = v.parse::<usize>().unwrap();
        t.push(val);
    }

    print_price(m, &mut btmap, &t, &mut writer);
}

pub fn print_price(
    m: usize,
    btmap: &mut BTreeMap<usize, usize>,
    t: &[usize],
    w: &mut BufWriter<StdoutLock>,
) {
    for i in 0..m {
        let v = t[i];

        let matched_key = btmap.range_mut(..=v).next_back().map(|(&k, _)| k);

        if let Some(key) = matched_key {
            writeln!(w, "{key}").unwrap();

            let entry = btmap.entry(key).and_modify(|count| *count -= 1);

            if let std::collections::btree_map::Entry::Occupied(e) = entry {
                if *e.get() == 0 {
                    e.remove();
                }
            }
        } else {
            writeln!(w, "-1").unwrap();
        }
    }
    w.flush().unwrap();
}
