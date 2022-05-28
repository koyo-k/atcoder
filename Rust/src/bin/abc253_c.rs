use std::{
    collections::BTreeMap,
    io::{self},
};

fn main() {
    let q = get_q();
    let query = get_query(q);
    let mut s: BTreeMap<i64, i64> = BTreeMap::new();
    for qu in query {
        if qu[0] == 1 {
            let x = qu[1];
            let count = s.entry(x).or_insert(0);
            *count += 1;
        } else if qu[0] == 2 {
            let [x, c] = [qu[1], qu[2]];
            let count = s.entry(x).or_insert(0);
            *count -= c;
            if s[&x] <= 0 {
                s.remove(&x);
            }
        } else if qu[0] == 3 {
            let min = s.iter().next().unwrap().0;
            let max = s.iter().rev().next().unwrap().0;
            println!("{}", max - min);
        }
    }
}

fn get_q() -> usize {
    let mut q = String::new();
    io::stdin().read_line(&mut q).unwrap();
    let mut iter = q.split_ascii_whitespace();
    return iter.next().unwrap().parse().unwrap();
}

fn get_query(q: usize) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = [].to_vec();
    for _ in 0..q {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let query = s.split_whitespace();
        let mut v: Vec<i64> = [].to_vec();
        for val in query {
            v.push(val.parse::<i64>().unwrap());
        }
        res.push(v);
    }
    return res;
}
