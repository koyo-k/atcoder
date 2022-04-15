use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [u32; n],
    }
    l.sort();
    let len = l.len();
    if len < 3 {
        println!("0");
        return;
    }
    let mut ans = 0;
    for i in 0..len - 2 {
        for j in i + 1..len - 1 {
            for k in j + 1..len {
                if l[i] + l[j] > l[k] && l[i] != l[j] && l[j] != l[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
