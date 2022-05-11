use proconio::input;

fn main() {
    input! {
        n: u32,
        d: [[u32; 2]; n]
    }

    let mut cnt = 0;
    for v in d {
        if v[0] == v[1] {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt >= 3 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
