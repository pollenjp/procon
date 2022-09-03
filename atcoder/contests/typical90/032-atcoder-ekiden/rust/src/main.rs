use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        in_n: usize,
        in_a: [[usize; in_n]; in_n],
        in_m: usize,
        in_x: [(usize, usize); in_m],
    }

    let mut st = HashSet::new();

    let in_x = in_x
        .into_iter()
        .map(|(x, y)| (x - 1, y - 1))
        .collect::<Vec<(usize, usize)>>();
    for &x in &in_x {
        let x2;
        if x.0 < x.1 {
            x2 = x;
        } else {
            x2 = (x.1, x.0);
        }
        st.insert(x2);
    }

    let mut minimum = std::usize::MAX;
    for perm in (0..in_n).permutations(in_n) {
        // check compatibility
        let mut compatible = true;
        let mut time = 0;
        for (i, &person) in perm[..in_n - 1].iter().enumerate() {
            let person2 = perm[(i + 1)];
            let pair;
            if person < person2 {
                pair = (person, person2);
            } else {
                pair = (person2, person);
            }

            if st.contains(&pair) {
                compatible = false;
                break;
            }

            time += in_a[person][i];
        }

        if compatible {
            let last_order = in_n - 1;
            let last_person = perm[last_order];
            time += in_a[last_person][last_order];
            minimum = std::cmp::min(minimum, time);
        }
    }

    if minimum == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", minimum);
    }
}
