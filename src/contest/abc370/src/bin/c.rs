use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut head = vec![];
    let mut tail = vec![];

    for i in 0..s.len() {
        let char_s = s.chars().nth(i).unwrap();
        let char_t = t.chars().nth(i).unwrap();
        if char_s == char_t {
            continue;
        }

        if char_s > char_t {
            head.push(i);
        } else {
            tail.push(i);
        }
    }

    println!("{}", head.len() + tail.len());
    let mut current: Vec<char> = s.chars().collect();
    for i in head {
        current[i] = t.chars().nth(i).unwrap();
        println!("{}", current.iter().join(""));
    }

    for i in tail.iter().rev() {
        current[*i] = t.chars().nth(*i).unwrap();
        println!("{}", current.iter().join(""));
    }
}
