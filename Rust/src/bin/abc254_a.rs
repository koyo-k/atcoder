use proconio::{input, marker::Chars};

fn main() {
    input! {n: Chars}
    println!("{}", n[n.len() - 2..].iter().collect::<String>());
}
