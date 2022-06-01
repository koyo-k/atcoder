use proconio::input;

fn main() {
    input! {
        h: i64, w: i64,
        a: [[i64;w];h]
    }
    let min = a.iter().map(|v| v.iter().min().unwrap()).min().unwrap();
    let sum = a.iter().map(|v| v.iter().sum::<i64>()).sum::<i64>();
    println!("{}", sum - min * h * w);
}
