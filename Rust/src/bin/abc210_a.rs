use proconio::input;

fn main() {
    input! {
        n: i64, a:i64, x: i64, y:i64
    }
    println!("{}", if n <= a { n * x } else { a * x + (n - a) * y })
}
