use im_rc::HashMap;
use itertools::Itertools;
use proconio::input;

fn check_order(relations: Vec<char>, order: &HashMap<char, usize>) -> bool {
    let pairs = vec![('A', 'B'), ('A', 'C'), ('B', 'C')];
    for i in 0..3 {
        let s = relations[i];
        let (a, b) = pairs[i];
        if (s == '<' && order[&a] > order[&b]) || (s == '>' && order[&a] < order[&b]) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        s_ab: char,
        s_ac: char,
        s_bc: char,
    }

    for perm in (0..3).permutations(3) {
        let mut order = HashMap::new();
        order.insert('A', perm[0]);
        order.insert('B', perm[1]);
        order.insert('C', perm[2]);

        if check_order(vec![s_ab, s_ac, s_bc], &order) {
            for c in ['A', 'B', 'C'].iter() {
                if order[c] == 1 {
                    println!("{}", c);
                    return;
                }
            }
        }
    }
}
