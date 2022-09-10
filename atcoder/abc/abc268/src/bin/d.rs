use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_s: [String; in_n],
        in_t: [String; in_m],
    };

    let total_s_num = in_s.iter().map(|s| s.len()).sum::<usize>();
    let additional_underscore_max_num = 16 - (in_n - 1 + total_s_num);

    let mut st = HashSet::new();

    for t in in_t {
        st.insert(t);
    }
    for underscore_perm in (0..additional_underscore_max_num + (in_n - 1)).combinations(in_n - 1) {
        for s_perm in in_s.iter().permutations(in_n) {
            let mut s = String::new();
            for i in 0..in_n {
                s.push_str(&s_perm[i]);
                if i < in_n - 1 {
                    if i == 0 {
                        s.push_str(&"_".repeat(underscore_perm[i] + 1));
                    } else {
                        s.push_str(&"_".repeat(underscore_perm[i] - underscore_perm[i - 1]));
                    }
                }
            }

            if s.len() < 3 {
                continue;
            }

            if !st.contains(&s) {
                println!("{}", s);
                return;
            }
        }
    }

    println!("-1");
}
