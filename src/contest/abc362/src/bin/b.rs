use proconio::input;

fn is_right_angle(v1: (i64, i64), v2: (i64, i64)) -> bool {
    v1.0 * v2.0 + v1.1 * v2.1 == 0
}

fn main() {
    input! {
        coords: [(i64, i64); 3],
    }

    let v1 = (coords[1].0 - coords[0].0, coords[1].1 - coords[0].1);
    let v2 = (coords[2].0 - coords[0].0, coords[2].1 - coords[0].1);
    let v3 = (coords[2].0 - coords[1].0, coords[2].1 - coords[1].1);

    if is_right_angle(v1, v2) || is_right_angle(v1, v3) || is_right_angle(v2, v3) {
        println!("Yes");
    } else {
        println!("No");
    }
}
