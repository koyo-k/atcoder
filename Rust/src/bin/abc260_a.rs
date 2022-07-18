use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut h = HashMap::new();
    for i in s {
        let c = h.entry(i).or_insert(0);
        *c += 1;
    }
    let mut ans: Vec<_> = h.into_iter().collect();
    ans.sort_by(|x, y| x.1.cmp(&y.1));
    if ans[0].1 == 1 {
        println!("{}", ans[0].0);
    } else {
        println!("-1");
    }
}
