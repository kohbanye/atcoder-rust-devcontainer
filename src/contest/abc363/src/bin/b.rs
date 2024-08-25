use proconio::input;

fn check(lengths: &Vec<usize>, t: &usize, p: &usize) -> bool {
    let mut cnt = 0;
    for length in lengths {
        if length >= t {
            cnt += 1;
        }
    }
    cnt >= *p
}

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut lengths: [usize; n],
    }

    let mut day = 0;
    loop {
        if check(&lengths, &t, &p) {
            println!("{}", day);
            return;
        }

        for length in lengths.iter_mut() {
            *length += 1;
        }
        day += 1;
    }
}
