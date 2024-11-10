use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let a = s.chars().next().unwrap();
    let b = s.chars().nth(1).unwrap();
    let c = s.chars().nth(2).unwrap();

    println!("{}{}{} {}{}{}", b, c, a, c, a, b);
}
