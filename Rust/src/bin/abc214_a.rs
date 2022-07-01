use proconio::input;

fn main() {
    input! {n: i64}
    println!(
        "{}",
        if n <= 125 {
            4
        } else if n <= 211 {
            6
        } else {
            8
        }
    );
}
