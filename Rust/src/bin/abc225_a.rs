use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let mut set = HashMap::new();
    for v in s {
        if !set.contains_key(&v) {
            set.insert(v, 1);
        }
    }
    println!(
        "{}",
        match set.len() {
            1 => 1,
            2 => 3,
            _ => 6,
        }
    );
}
