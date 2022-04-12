use proconio::{input, marker::Chars};

fn main() {
    input! {
        abc: Chars,
    }
    println!(
        "{:?}",
        abc.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() * 111
    );
}
