use proconio::input;

fn main() {
    input! {
        h: i64, w: i64,
        r:i64 , c:i64,
    }
    println!(
        "{}",
        4 - [h == r, r == 1, w == c, c == 1]
            .iter()
            .map(|f| *f as i64)
            .sum::<i64>()
    )
}
