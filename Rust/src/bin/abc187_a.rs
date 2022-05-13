use proconio::{input, marker::Chars};

fn main() {
    input! {a: Chars, b: Chars}
    println!("{}", digit_sum(a).max(digit_sum(b)));
}

fn digit_sum(x: Vec<char>) -> usize {
    return x
        .iter()
        .fold(0, |p, n| p + n.to_digit(10).unwrap() as usize);
}
