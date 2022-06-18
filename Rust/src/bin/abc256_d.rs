use proconio::input;

struct P {
    key: String,
    val: i64,
}

fn main() {
    input! {
        n: usize,
        rf: [[i64; 2];n],
    }
    let mut s = vec![];
    for i in 0..n {
        s.push(P {
            key: "l".to_string(),
            val: rf[i][0],
        });
        s.push(P {
            key: "r".to_string(),
            val: rf[i][1],
        });
    }
    s.sort_by(|a, b| cmp(a, b));
    let mut l = 0;
    let mut r = 0;
    let mut tmp = 0;
    for v in s.iter() {
        if l == r {
            tmp = v.val;
        }
        if v.key == "l".to_string() {
            l += 1;
        } else {
            r += 1;
            if l == r {
                println!("{} {}", tmp, v.val);
            }
        }
    }
}

fn cmp(a: &P, b: &P) -> std::cmp::Ordering {
    if a.val.cmp(&b.val) != std::cmp::Ordering::Equal || a.key == b.key {
        a.val.cmp(&b.val)
    } else {
        if a.key == "l" {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
