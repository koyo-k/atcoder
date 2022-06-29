use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = vec![];
    for v in s {
        ans.push(f(v));
    }
    ans.reverse();
    println!("{}", ans.into_iter().collect::<String>());
}

fn f(c: char) -> char {
    return match c {
        '6' => '9',
        '9' => '6',
        _ => c,
    };
}
