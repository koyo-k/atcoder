fn main() {
    let (x1, y1) = getlines();
    let (x2, y2) = getlines();
    let (x3, y3) = getlines();
    let (x4, y4);
    if x1 == x2 {
        x4 = x3;
    } else if x1 == x3 {
        x4 = x2;
    } else {
        x4 = x1;
    }
    if y1 == y2 {
        y4 = y3;
    } else if y1 == y3 {
        y4 = y2;
    } else {
        y4 = y1;
    }
    println!("{} {}", x4, y4);
}

fn getlines() -> (i64, i64) {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    return (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );
}
