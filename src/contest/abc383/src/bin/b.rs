use itertools::iproduct;
use proconio::input;

fn count_humid_floor(
    h: usize,
    w: usize,
    d: i32,
    map: Vec<String>,
    coord1: (usize, usize),
    coord2: (usize, usize),
) -> i32 {
    let mut count = 0;
    for (i, j) in iproduct!(0..h, 0..w) {
        if map[i].chars().nth(j).unwrap() == '#' {
            continue;
        }
        let dist1 = (coord1.0 as i32 - i as i32).abs() + (coord1.1 as i32 - j as i32).abs();
        let dist2 = (coord2.0 as i32 - i as i32).abs() + (coord2.1 as i32 - j as i32).abs();
        if dist1 <= d || dist2 <= d {
            count += 1;
        }
    }
    count
}

fn main() {
    input! {
        h: usize,
        w: usize,
        d: i32,
        map: [String; h],
    }

    let mut max_count = 0;
    for (i, j) in iproduct!(0..h, 0..w) {
        for (k, l) in iproduct!(0..h, 0..w) {
            if i == k && j == l
                || map[i].chars().nth(j).unwrap() == '#'
                || map[k].chars().nth(l).unwrap() == '#'
            {
                continue;
            }
            max_count = max_count.max(count_humid_floor(h, w, d, map.clone(), (i, j), (k, l)));
        }
    }

    println!("{:?}", max_count);
}
