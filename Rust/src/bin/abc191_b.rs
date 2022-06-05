use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
<<<<<<< HEAD
        n: usize, x: i64,
        mut a: [i64;n],
=======
        n: usize, x: i128,
        mut a: [i128;n],
>>>>>>> b7b9eddd7d14da367a395b042afa4c04f9d46b52
    };
    println!("{}", a.into_iter().filter(|a| *a != x).join(" "));
}
