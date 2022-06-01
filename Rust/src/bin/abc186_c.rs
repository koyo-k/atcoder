use proconio::input;

fn main() {
    input! {n: i64}
    let mut ans = n.clone();
    for i in 1..n + 1 {
        let x = format!("{num:o}", num = i);
        if i.to_string().contains("7") || x.contains("7") {
            ans -= 1;
        }
    }
    println!("{}", ans);
}
