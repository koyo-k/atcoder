use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }
    let l: HashSet<String> = s.iter().cloned().collect();
    for i in 0..n {
        if l.contains(&s[i]) && l.contains(&("!".to_string() + &s[i])) {
            println!("{}", s[i]);
            return;
        }
    }
    println!("satisfiable");
}
