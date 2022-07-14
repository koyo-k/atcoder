use proconio::input;

fn main() {
    input! {s: String}
    let mut ans = "".to_string();
    while ans.len() < 6 {
        ans.push_str(s.as_str());
    }
    println!("{}", ans);
}
