use proconio::input;

fn main() {
    input! {n: u32}
    if n < 2 || n > 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
