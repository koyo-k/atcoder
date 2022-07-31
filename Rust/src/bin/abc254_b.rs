use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n:i64};
    f(vec![1], n);
}

fn f(n: Vec<i64>, l: i64) {
    let mut nn = vec![];
    println!("{}", n.iter().map(|x| x.to_string()).join(" "));
    for i in 0..=n.len() {
        if i == 0 || i == n.len() {
            nn.push(1);
        } else {
            nn.push(n[i - 1] + n[i]);
        }
    }
    if n.len() != (l as usize) {
        f(nn, l);
    }
}
