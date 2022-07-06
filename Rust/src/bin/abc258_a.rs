use proconio::input;
fn main() {
    input! {
        k: i64
    }
    println!("{}:{:>02}", 21 + k / 60, k % 60);
}
