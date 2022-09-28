use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        in_a: [usize; 5],
    };

    let mut st = BTreeSet::new();

    for val in in_a {
        st.insert(val);
    }

    println!("{}", st.len());
}
