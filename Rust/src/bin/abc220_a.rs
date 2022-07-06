use proconio::input;

fn main() {
    input! {
        a: i64, b:i64, c:i64
    }
    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
