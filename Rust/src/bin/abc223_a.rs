use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    println!("{}", if x > 0 && x % 100 == 0 { "Yes" } else { "No" });
}
