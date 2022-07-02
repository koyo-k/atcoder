use itertools::Itertools;
use proconio::input;

fn main() {
    input! {s: String}
    let (x, y) = s
        .split(".")
        .map(|s| s.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    if y <= 2 {
        println!("{}-", x);
    } else if y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
