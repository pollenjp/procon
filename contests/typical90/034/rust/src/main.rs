use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
        in_a: [usize; in_n],
    }

    let mut mp = HashMap::<usize, usize>::new();

    let mut max_len = 0;
    let mut left_idx = 0;

    for (right_idx, &a) in in_a.iter().enumerate() {
        if mp.contains_key(&a) {
            mp.insert(a, mp[&a] + 1);
        } else {
            mp.insert(a, 1);
        }

        while mp.len() > in_k {
            // dbg!(&mp);
            let left_a = in_a[left_idx];
            mp.insert(left_a, mp[&left_a] - 1);
            if mp[&left_a] == 0 {
                mp.remove(&left_a);
            }
            left_idx += 1;
        }
        max_len = std::cmp::max(max_len, right_idx - left_idx + 1);
    }

    println!("{}", max_len);
}
