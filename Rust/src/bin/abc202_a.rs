use proconio::input;

fn main() {
    input! {
        d: [i64; 3],
    }
    println!("{}", d.iter().map(|x| (7 - x)).fold(0, |x, y| x + y));
}
