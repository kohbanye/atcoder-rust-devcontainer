use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    if n < 3 {
        println!("0");
        return;
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut cnt = 0;
    for i in 0..n - 2 {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
