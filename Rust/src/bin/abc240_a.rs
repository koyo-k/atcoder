use proconio::input;
fn main() {
    input! {
        a: i64,
        b: i64,
    }
    if b - a == 1 || b - a == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
