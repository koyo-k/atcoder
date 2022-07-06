use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
    }
    println!("{}", a.iter().map(|n| (n - 10).max(0)).sum::<i64>());
}
