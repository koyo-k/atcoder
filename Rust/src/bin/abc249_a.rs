use proconio::input;

fn main() {
    input! {
        a:i64, b:i64, c:i64, d:i64, e:i64, f:i64, x:i64,
    }
    let takahashi = distance(a, b, c, x);
    let aoki = distance(d, e, f, x);
    println!(
        "{}",
        if takahashi > aoki {
            "Takahashi"
        } else if aoki > takahashi {
            "Aoki"
        } else {
            "Draw"
        }
    );
}

fn distance(a: i64, b: i64, c: i64, x: i64) -> i64 {
    return b * (a * (x / (a + c)) + a.min(x % (a + c)));
}
