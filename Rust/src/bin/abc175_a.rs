use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    if s == "SSR" || s == "SRS" || s == "RSS" || s == "RSR" {
        ans = 1;
    } else if s == "SRR" || s == "RRS" {
        ans = 2;
    } else if s == "RRR" {
        ans = 3;
    }
    println!("{}", ans);
}
