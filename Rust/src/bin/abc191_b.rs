use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, x: i64,
        mut a: [i64;n],
    };
    println!("{}", a.into_iter().filter(|a| *a != x).join(" "));
}
