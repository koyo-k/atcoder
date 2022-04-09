use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        a: usize,
        b: usize,
    }
    let sa = s[a - 1];
    let sb = s[b - 1];
    s[a - 1] = sb;
    s[b - 1] = sa;
    let ans: String = s.iter().collect();
    println!("{}", &ans);
}
