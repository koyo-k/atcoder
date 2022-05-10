use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    println!(
        "{}",
        [a * c, a * d, b * c, b * d]
            .iter()
            .fold(std::i64::MIN, |x, y| x.max(*y))
    );
}
