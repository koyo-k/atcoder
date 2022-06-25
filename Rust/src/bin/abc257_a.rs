use proconio::input;

fn main() {
    input! {n:usize, x:usize}
    let mut count = 0;
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().into_iter() {
        for _ in 0..n {
            count += 1;
            if x == count {
                print!("{}", c)
            }
        }
    }
}
