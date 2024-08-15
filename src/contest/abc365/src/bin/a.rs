use proconio::input;

fn main() {
    input! {
        year: i32,
    }

    let ans = if year % 4 != 0 {
        365
    } else if year % 100 != 0 {
        366
    } else if year % 400 != 0 {
        365
    } else {
        366
    };

    println!("{}", ans);
}
