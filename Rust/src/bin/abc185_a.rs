use proconio::input;

fn main() {
    input! {
        a: [u32;4]
    }
    println!("{}", a.iter().fold(100, |x, y| x.min(*y)));
}
