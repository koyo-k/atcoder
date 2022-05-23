use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: i64, mut x: i64,
        s: Chars,
    }
    println!(
        "{}",
        s.iter()
            .fold(x, |l, r| if *r == 'x' { (l - 1).max(0) } else { l + 1 })
    )
}
