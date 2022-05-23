use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let sq = (n as f64).sqrt() as u64 + 1;
    let mut ans = vec![];
    for i in 1..sq + 1 {
        if n as u64 % i == 0 {
            ans.push(i);
            ans.push(n / i);
        }
    }
    ans.sort();
    ans.dedup();
    ans.iter().for_each(|i| println!("{}", i));
}
