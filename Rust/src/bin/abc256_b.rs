use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut ans = 0;
    let mut l = vec![0, 0, 0, 0];
    for i in a.iter() {
        l[0] = 1;
        for j in (0..4).rev() {
            if l[j] == 0 {
                continue;
            }
            l[j] = 0;
            if j + i > 3 {
                ans += 1;
            } else {
                l[j + i] = 1;
            }
        }
    }
    println!("{}", ans);
}
