use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
    }
    let m = 10_i64.pow(9) + 7;
    let sum = a.iter().fold(0, |p, v| (p + v) % m);
    println!(
        "{}",
        a.iter().fold(0, |p, v| (v * (sum - v + m) + p) % m) * 500000004 % m
    );
}
