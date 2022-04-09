use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    println!("{}", (x * (12800000.0 + x)).sqrt());
}
