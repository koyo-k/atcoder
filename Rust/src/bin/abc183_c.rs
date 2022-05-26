use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: i64,
        m: [[i64;n];n],
    }
    let mut ans = 0;
    for mut perm in (1..n).permutations(n - 1) {
        let mut p = vec![0];
        p.append(&mut perm);
        p.push(0);
        let mut tmp = 0;
        for i in 0..n {
            tmp += m[p[i]][p[i + 1]];
        }
        if tmp == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
