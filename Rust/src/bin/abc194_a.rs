use proconio::input;

fn main() {
    input! {
        a: u64, b: u64,
    }
    let c = a + b;
    if c >= 15 && b >= 8 {
        println!("1");
    } else if c >= 10 && b >= 3 {
        println!("2");
    } else if c >= 3 {
        println!("3");
    } else {
        println!("4");
    }
}
