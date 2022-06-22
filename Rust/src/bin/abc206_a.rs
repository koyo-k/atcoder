use proconio::input;

fn main() {
    input! {n: i64}
    println!(
        "{}",
        if n < 191 {
            "Yay!"
        } else if n == 191 {
            "so-so"
        } else {
            ":("
        }
    );
}
