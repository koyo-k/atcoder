use proconio::input;

fn main() {
    input! {
        d: u32,
        t: u32,
        s: u32,
    }

    if d <= s * t {
        println!("Yes");
    } else {
        println!("No");
    }
}
