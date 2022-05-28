use proconio::input;

fn main() {
    input! {a: i64, b: i64, c: i64}
    print!(
        "{}",
        if (a <= b && b <= c) || (c <= b && b <= a) {
            "Yes"
        } else {
            "No"
        }
    );
}
