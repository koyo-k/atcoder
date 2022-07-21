use proconio::input;

fn main() {
    input! {
        s: [String;2],
    }
    if (s[0] == ".#" && s[1] == "#.") || (s[0] == "#." && s[1] == ".#") {
        println!("No");
        return;
    }
    println!("Yes");
}
