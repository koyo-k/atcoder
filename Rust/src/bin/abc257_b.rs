use proconio::input;

fn main() {
    input! {
        n: i64, k: usize, q: usize,
        a: [i64; k],
        l: [i64; q],
    }
    let mut b = vec![];
    for _ in 0..n {
        b.push(0);
    }
    for (i, v) in a.iter().enumerate() {
        b[(v - 1) as usize] = (i + 1) as i64;
    }
    for v in l {
        for (k, val) in b.clone().iter().enumerate() {
            if v == *val && (k as i64) < n - 1 && b[k + 1] == 0 {
                b[k] = 0;
                b[k + 1] = *val;
            }
        }
    }
    let mut cnt = 0;
    for (i, v) in b.iter().enumerate() {
        if *v == 0 {
            continue;
        }
        if cnt == 0 {
            print!("{}", i + 1)
        } else {
            print!(" {}", i + 1)
        }
        cnt += 1;
    }
}
