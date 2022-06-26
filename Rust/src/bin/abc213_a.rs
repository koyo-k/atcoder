use std::ops::BitXor;

use proconio::input;

fn main() {
    input! {
        a: i64, b: i64
    }
    for i in 0..256 {
        if a.bitxor(i) == b {
            return println!("{}", i);
        }
    }
}
