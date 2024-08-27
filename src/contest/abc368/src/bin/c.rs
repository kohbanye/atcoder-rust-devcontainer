use proconio::input;

fn turns_to_kill(hp: i64, current_turn: i64) -> i64 {
    let remainig_attacks = match (current_turn + 1) % 3 {
        0 => {
            vec![3, 1, 1]
        }
        1 => {
            vec![1, 1, 3]
        }
        2 => {
            vec![1, 3, 1]
        }
        _ => {
            unreachable!();
        }
    };

    let mut turns = 0;
    turns += hp / 5 * 3;
    let mut remaining_hp = hp % 5;
    for &attack in remainig_attacks.iter() {
        if remaining_hp <= 0 {
            break;
        }
        if remaining_hp <= attack {
            turns += 1;
            break;
        }
        turns += 1;
        remaining_hp -= attack;
    }

    turns
}

fn main() {
    input! {
        n: usize,
        hps: [i64; n],
    }

    let mut turns = 0;
    for hp in hps {
        turns += turns_to_kill(hp, turns);
    }

    println!("{:?}", turns);
}
