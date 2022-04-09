use proconio::input;

fn main() {
    input! {n: i64}
    let p = 2_i64.pow(31);
    if -p <= n && n < p {
        println!("Yes");
    } else {
        println!("No");
    }
}
