use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut top = Mount {
        s: "".to_string(),
        t: 0,
    };
    let mut second = Mount {
        s: "".to_string(),
        t: 0,
    };
    for _ in 0..n {
        input! {s: String, t: i64}
        if top.t <= t {
            second = Mount { s: top.s, t: top.t };
            top = Mount { s, t };
        } else if second.t <= t {
            second = Mount { s, t };
        }
    }
    println!("{}", second.s);
}

struct Mount {
    s: String,
    t: i64,
}
