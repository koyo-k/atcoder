use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }
    if s[s.len() - 1].to_string() == "s" {
        println!("{}es", s.iter().collect::<String>());
    } else {
        println!("{}s", s.iter().collect::<String>());
    }
}
