use proconio::input;

fn main() {
    input! {
        n: usize, s: i64, d: i64,
        m: [[i64;2];n]
    };
    for sp in m.iter() {
        if sp[0] < s && sp[1] > d {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
