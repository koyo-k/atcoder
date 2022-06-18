use proconio::input;

fn main() {
    input! {h1: i64, h2: i64, h3: i64, w1: i64, w2: i64, w3: i64}
    let mut ans = 0;
    for i in 1..=28 {
        for j in 1..=28 {
            for k in 1..=28 {
                for l in 1..=28 {
                    let p02 = h1 - (i + j);
                    let p12 = h2 - (k + l);
                    let p20 = w1 - (i + k);
                    let p21 = w2 - (j + l);
                    let p22 = h3 - (p20 + p21);
                    if p02 > 0
                        && p12 > 0
                        && p20 > 0
                        && p21 > 0
                        && p22 > 0
                        && p22 == w3 - (p02 + p12)
                    {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
