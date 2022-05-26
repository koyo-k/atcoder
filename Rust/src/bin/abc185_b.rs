use proconio::input;

fn main() {
    input! {
        n: i64, m: i64, t: i64,
        c: [[i64;2];m]
    }
    let mut time = 0;
    let mut ans = n.clone();
    for v in c.iter() {
        let (a, b) = (v[0], v[1]);
        ans -= a - time;
        if ans <= 0 {
            println!("No");
            return;
        }
        ans += b - a;
        ans = ans.min(n);
        time = b;
    }
    if ans - t + time <= 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
