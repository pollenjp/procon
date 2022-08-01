use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_uv: [(usize, usize); in_m],
    }

    let mut st = HashSet::new();

    for &(u, v) in &in_uv {
        let u = u - 1;
        let v = v - 1;
        if u < v {
            st.insert((u, v));
        } else {
            st.insert((v, u));
        }
    }

    let mut cnt = 0;
    for a in 0..in_n - 2 {
        for b in a..in_n - 1 {
            for c in b..in_n {
                if st.contains(&(a, b)) && st.contains(&(b, c)) && st.contains(&(a, c)) {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
