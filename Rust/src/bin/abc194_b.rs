use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [[i64; 2]; n],
    }
    let mut ans = std::i64::MAX;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans = ans.min(t[i][0] + t[j][1]);
            } else {
                ans = ans.min(t[i][0].max(t[j][1]));
            }
        }
    }
    println!("{}", ans);
}
