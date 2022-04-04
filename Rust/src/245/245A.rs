fn main() {
    let (a, b, c, d) = getlines();
    if a < c || a == c && b <= d {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn getlines() -> (i64, i64, i64, i64) {
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
