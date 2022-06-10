use proconio::input;

fn main() {
    input! {
        x: String
    }
    let mut ans = "".to_string();
    for v in x.chars() {
        if v == '.' {
            break;
        }
        ans += &v.to_string();
    }
    println!("{}", ans);
}
