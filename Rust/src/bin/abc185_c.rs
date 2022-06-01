use proconio::input;

fn main() {
    input! {l: u64}
    let mut ans = 1;
    let mut eleven = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    for i in l - 11..l {
        ans *= i;
        let mut nouse = vec![];
        for j in eleven.clone() {
            if ans % j == 0 {
                ans /= j;
            } else {
                nouse.push(j)
            }
        }
        eleven = nouse;
    }
    println!("{}", ans);
}
