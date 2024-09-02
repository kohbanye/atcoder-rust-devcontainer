use proconio::input;

struct Position {
    left: usize,
    right: usize,
}

fn main() {
    input! {
        n: usize,
        a: [(usize, char); n],
    }

    let mut current_position = Position { left: 0, right: 0 };
    let mut fatigue = 0;
    for (position, hand) in &a {
        if *hand == 'L' {
            if current_position.left != 0 {
                fatigue += (*position as i32 - current_position.left as i32).abs();
            }
            current_position.left = *position;
        } else {
            if current_position.right != 0 {
                fatigue += (*position as i32 - current_position.right as i32).abs();
            }
            current_position.right = *position;
        }
    }

    println!("{}", fatigue);
}
