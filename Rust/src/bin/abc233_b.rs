use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
        mut s: Chars,
    }

    s[l - 1..r].reverse();
    println!("{}", s.iter().collect::<String>());
}
