use proconio::input;

fn main() {
    input! {
        mut n: i64, k: i64,
    }
    for _ in 0..k {
        n = f(n);
    }
    println!("{}", n);
}

fn g1(x: i64) -> i64 {
    let mut res = vec![];
    for i in x.to_string().split("") {
        res.push(i.to_string());
    }
    res.sort();
    res.reverse();
    return res.concat().parse::<i64>().unwrap();
}

fn g2(x: i64) -> i64 {
    let mut res = vec![];
    for i in x.to_string().split("") {
        res.push(i.to_string());
    }
    res.sort();
    return res.concat().parse::<i64>().unwrap();
}

fn f(x: i64) -> i64 {
    return g1(x) - g2(x);
}
