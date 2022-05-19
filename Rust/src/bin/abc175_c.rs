use proconio::input;

fn main() {
    input! {mut x: i64, k: i64, d: i64}
    x = x.abs();
    if x / k >= d {
        println!("{}", x - k * d);
    } else if (k - ((x - x % d) / d)) % 2 == 0 {
        println!("{}", x % d);
    } else {
        println!("{}", d - x % d);
    }
}
