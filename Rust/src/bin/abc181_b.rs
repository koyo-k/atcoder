use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [[usize;2]; n],
    }
    println!("{:?}", x.into_iter().map(|v| f(v)).sum::<usize>());
}

fn f(v: Vec<usize>) -> usize {
    let x = v[0] - 1;
    let y = v[1];
    return (y * (y + 1) - x * (x + 1)) / 2;
}
