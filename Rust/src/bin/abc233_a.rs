use proconio::input;
use std::cmp;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    println!("{}", cmp::max(0, (y - x + 9) / 10));
}
