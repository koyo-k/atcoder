fn main() {
    let (v, a, b, c) = getline();
    let sum = a + b + c;
    if v % sum < a {
        println!("{}", "F");
    } else if v % sum < (a + b) {
        println!("{}", "M");
    } else {
        println!("{}", "T");
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
