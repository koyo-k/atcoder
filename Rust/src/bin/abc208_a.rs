use proconio::input;

fn main() {
    input! {a: i64, b:i64}
    println!("{}", if a <= b && b <= 6 * a { "Yes" } else { "No" });
}
