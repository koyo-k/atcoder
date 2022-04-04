fn main() {
    let (a, b, c, x) = getline();
    if x <= a {
        println!("{:?}", 1);
    } else if x <= b {
        println!("{:?}", c as f64 / (b - a) as f64);
    } else {
        println!("{:?}", 0);
    }
}

fn getline() -> (i64, i64, i64, i64) {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    return (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );
}
