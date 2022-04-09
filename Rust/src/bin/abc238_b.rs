use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let sq = (1_f64 / (a.powi(2) + b.powi(2))).sqrt();
    let x = a * sq;
    let y = b * sq;
    println!("{} {}", x, y);
}
