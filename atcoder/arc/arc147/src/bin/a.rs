use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    };

    let mut st = BTreeSet::new();
    for i in 0..in_n {
        st.insert((in_a[i], i));
    }

    let mut cnt = 0;
    while st.len() > 1 {
        let &a_i = st.iter().next_back().unwrap();
        let &a_j = st.iter().next().unwrap();
        let x = a_i.0 % a_j.0;
        st.remove(&a_i);
        if x != 0 {
            st.insert((x, a_i.1));
        }
        cnt += 1;
    }

    println!("{}", cnt);
}
