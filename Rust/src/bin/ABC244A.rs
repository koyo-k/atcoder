fn main() {
    getline();
    let mut s = getline();
    s.pop();
    let ans = s.pop();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("None"),
    }
}

fn getline() -> String {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}
