use proconio::input;

fn main() {
    input! {s:i64,mut t:i64, mut x:i64}
    for i in s..=s + 24 {
        if i % 24 == t {
            println!("No");
            return;
        }
        if i % 24 == x {
            println!("Yes");
            return;
        }
    }
}
