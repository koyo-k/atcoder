use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = a[0];
    let mut max_cnt = 0;
    for i in 2..1001 {
        let mut cnt = 0;
        for v in a.iter() {
            if v % i == 0 {
                cnt += 1;
            }
        }
        if cnt > max_cnt {
            max_cnt = cnt;
            ans = i;
        }
    }
    println!("{}", ans);
}
