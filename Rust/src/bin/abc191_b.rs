use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, x: i128,
        mut a: [i128;n],
    };
    println!("{}", a.into_iter().filter(|a| *a != x).join(" "));
}
