use proconio::input;

fn main() {
    input! {
        x: u32
    }
    println!("{}", (x + 1) % 2);
}
