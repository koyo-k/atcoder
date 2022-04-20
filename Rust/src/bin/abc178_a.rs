use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }
    if n.into_iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
