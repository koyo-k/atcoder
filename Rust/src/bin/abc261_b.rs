use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars;n],
    }
    for i in 0..n - 1 {
        for j in i + 1..n {
            let l = a[i][j];
            let r = a[j][i];
            if l == r && (l == 'W' || r == 'L') || l != r && (l == 'D' || r == 'D') {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}
