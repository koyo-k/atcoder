use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!(
        "{}",
        "01234567890"
            .chars()
            .find(|v| s.iter().all(|n| n != v))
            .unwrap()
    );
}
