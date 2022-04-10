use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    print!("0");
    s.pop();
    for v in s {
        print!("{}", v);
    }
}
