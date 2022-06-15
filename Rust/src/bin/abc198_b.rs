use proconio::input;

fn main() {
    input! {mut n: String}
    for _ in (0..n.len()).rev() {
        if n.ends_with("0") {
            n.pop();
        } else {
            break;
        }
    }
    let rev = n.chars().rev().collect::<String>();
    if n == rev {
        println!("Yes");
    } else {
        println!("No");
    }
}
