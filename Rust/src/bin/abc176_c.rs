use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut ans = 0;
    for i in 1..n {
        let step = 0_i64.max(a[i - 1] - a[i]);
        ans += step;
        a[i] += step;
    }
    println!("{}", ans);
}
