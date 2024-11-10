use proconio::input;

fn main() {
    input! {
        _: usize,
        k: usize,
        s: String,
    }

    let teeth = s.chars().collect::<Vec<char>>();
    let mut cnt = 0;
    let mut ans = 0;
    for tooth in teeth {
        if tooth == 'O' {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt >= k {
            ans += 1;
            cnt = 0;
        }
    }

    println!("{:?}", ans);
}
