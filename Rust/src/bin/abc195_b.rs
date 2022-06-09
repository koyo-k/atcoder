use proconio::input;

fn main() {
    input! {
        a: f64, b: f64, w: f64
    }
    let min = (1000.0 * w / b).ceil();
    let max = (1000.0 * w / a).floor();
    if min > max {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min, max);
}
