use proconio::input;

fn main() {
    input! {
        n: i64,
        x: [i64; n]
    }
    println!("{}", x.iter().map(|v| v.abs()).sum::<i64>());
    println!("{}", x.iter().map(|v| (v * v) as f64).sum::<f64>().sqrt());
    println!("{}", x.iter().fold(0_i64, |x1, x2| x1.abs().max(x2.abs())));
}
