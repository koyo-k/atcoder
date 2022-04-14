use proconio::input;

fn main() {
    let mut ans = 0;
    input! {
        n: i64,
        d: i64,
    }
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }

        if d * d >= x * x + y * y {
            ans += 1;
        }
    }
    println!("{}", ans);
}
