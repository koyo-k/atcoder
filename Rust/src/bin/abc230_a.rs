use proconio::input;

fn main() {
    input! {n: i64}
    println!("AGC{:>03}", if n < 42 { n } else { n + 1 })
}
