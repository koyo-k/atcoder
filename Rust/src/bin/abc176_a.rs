use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        t: u32,
    }

    println!("{}", (n + x - 1) / x * t);
}
