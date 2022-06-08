use proconio::input;

fn main() {
    input! {
        n: u32,
        r: [i64; 2_i64.pow(n)],
    }
    let n2 = 2_i64.pow(n);
    let mid = (n2 / 2) as usize;
    let left = r[..mid].iter().fold(0_i64, |acc, x| acc.max(*x));
    let right = r[mid..].iter().fold(0_i64, |acc, x| acc.max(*x));
    let second = left.min(right);
    for (i, val) in r.iter().enumerate() {
        if val == &second {
            println!("{}", i + 1);
            return;
        }
    }
}
