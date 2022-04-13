use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    println!("{}", 0.max((y - x + 9) / 10));
}
