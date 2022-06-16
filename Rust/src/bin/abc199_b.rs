use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    println!(
        "{}",
        0.max(b.iter().min().unwrap() - a.iter().max().unwrap() + 1)
    );
}
