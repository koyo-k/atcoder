use proconio::input;
fn main() {
    input! {
        x: [usize; 10],
    }
    println!("{:?}", x[x[x[0]]]);
}
