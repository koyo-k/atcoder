use proconio::input;

fn main() {
    input! {
        n:i64,k:i64,a:i64
    }
    println!("{}", (k + a - 2) % n + 1);
}
