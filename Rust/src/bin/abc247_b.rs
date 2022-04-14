use std::{collections::HashMap, process::exit};

use proconio::input;

fn main() {
    input! {
        n: i64,
        mut w: [[String; 2]; n],
    }
    let mut ans: bool = true;
    let mut map = HashMap::new();
    // map.insert(&w[0][0], 0);
    for v in &mut w[1..] {
        if map.contains_key(&v[0]) {
            map.insert(&v[1], 0);
        } else if map.contains_key(&v[1]) {
            map.insert(&v[0], 0);
        } else {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
        exit(1);
    }
    // map.clear();
    // map.insert(&w[0][1], 0);
    // for v in &mut w[1..] {
    //     if map.contains_key(&v[0]) {
    //         map.insert(&v[1], 0);
    //     } else if map.contains_key(&v[1]) {
    //         map.insert(&v[0], 0);
    //     } else {
    //         println!("No");
    //         exit(1);
    //     }
    // }
    println!("Yes");
}
