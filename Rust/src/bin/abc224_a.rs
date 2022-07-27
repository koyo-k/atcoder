use proconio::input;
use regex::Regex;

fn main() {
    input! {s: String}
    let re = Regex::new(r"er$").unwrap();
    println!("{}", if re.is_match(&s) { "er" } else { "ist" });
}
