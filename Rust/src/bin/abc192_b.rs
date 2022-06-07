use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    for (i, c) in enumerate(s) {
        if i % 2 == 0 && !"abcdefghijklmnopqrstuvwxyz".contains(c)
            || i % 2 == 1 && !"ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
