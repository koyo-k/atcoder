use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = t.len();
    for i in 0..s.len() - t.len() + 1 {
        let mut tmp = 0;
        for (v1, v2) in t.iter().zip(s[i..i + t.len()].iter()) {
            if v1 != v2 {
                tmp += 1;
            }
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
