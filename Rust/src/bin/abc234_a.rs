use proconio::input;

fn main() {
    input! {
        x: i64
    }
    println!("{}", func(func(func(x) + x) + func(func(x))));
}

fn func(x: i64) -> i64 {
    return x.pow(2) + 2 * x + 3;
}
