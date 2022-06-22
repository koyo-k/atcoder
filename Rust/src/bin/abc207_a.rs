use proconio::input;

fn main() {
    input! {
        mut c: [i64; 3]
    }
    c.sort();
    println!("{}", c[1..].iter().sum::<i64>());
}
