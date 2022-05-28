use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars;h]
    }
    let mut ans: Vec<[i64; 2]> = [].to_vec();
    for x in 0..h {
        for y in 0..w {
            if s[x][y].to_string() == "o" {
                ans.push([x as i64, y as i64])
            }
        }
    }
    println!(
        "{}",
        (ans[0][1] - ans[1][1]).abs() + (ans[0][0] - ans[1][0]).abs()
    );
}
