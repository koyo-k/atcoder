use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [[f64; 2];n],
    }
    for v1 in (&p).into_iter() {
        for v2 in (&p).into_iter() {
            for v3 in (&p).into_iter() {
                if v1 == v2 || v2 == v3 || v1 == v3 {
                    continue;
                }
                if v1[0] == v2[0] && v2[0] == v3[0] {
                    println!("Yes");
                    return;
                }
                if (v1[1] - v2[1]) / (v1[0] - v2[0]) == (v2[1] - v3[1]) / (v2[0] - v3[0]) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
