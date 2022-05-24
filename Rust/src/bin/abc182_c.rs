use proconio::{input, marker::Chars};

fn main() {
    input! {mut n: Chars}
    println!("{}", f(n, 0, 0))
}

fn f(mut n: Vec<char>, sum: i32, res: i32) -> i32 {
    if n.len() == 0 {
        if sum != 0 && sum % 3 == 0 {
            return res;
        }
        return -1 as i32;
    } else {
        let num = n.pop().unwrap().to_string().parse::<i32>().unwrap();
        let del = f(n.clone(), sum, res + 1);
        let notdel = f(n.clone(), sum + num, res);
        if del != -1 && notdel != -1 {
            return del.min(notdel);
        } else if del != -1 {
            return del;
        } else if notdel != -1 {
            return notdel;
        } else {
            return -1;
        }
    }
}
