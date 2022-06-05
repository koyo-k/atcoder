use proconio::input;

fn main() {
    input! {
        n: usize, mut x: f64,
        a: [[f64;2];n],
    }
    x = x * 100.0;
    for i in 0..n {
        x -= a[i][0] * a[i][1];
        if x < 0.0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
