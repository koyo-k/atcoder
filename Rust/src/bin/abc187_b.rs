use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[i64;2];n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if c[i][0] == c[j][0] {
                continue;
            }
            let d = (c[j][1] - c[i][1]) as f64 / (c[j][0] - c[i][0]) as f64;
            if -1.0 <= d && d <= 1.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
