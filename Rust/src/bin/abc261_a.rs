use proconio::input;

fn main() {
    input! {l1: i64, r1:i64, l2: i64, r2: i64}
    println!("{}", (r2.min(r1) - l1.max(l2)).max(0));
}
