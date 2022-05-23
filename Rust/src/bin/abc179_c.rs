use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        for j in 1..n / i + 1 {
            if i * j < n {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
