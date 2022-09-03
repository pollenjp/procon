use proconio::input;
use std::collections::BTreeSet;
use std::collections::HashSet;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_a: [usize; in_n],
        in_uv: [(usize, usize); in_m],
    }

    let mut graph = vec![HashSet::new(); in_n];
    for (u, v) in in_uv {
        graph[u - 1].insert(v - 1);
        graph[v - 1].insert(u - 1);
    }

    let mut st = BTreeSet::new();
    let mut st_save = vec![];
    for i in 0..in_n {
        let mut cost = 0;
        for &j in &graph[i] {
            cost += in_a[j];
        }

        let x = (cost, -(in_a[i] as i64), i);
        st.insert(x);
        st_save.push(x);
    }

    let mut cost = std::usize::MIN;
    for _ in 0..in_n {
        let &(c, c_self, i) = st.iter().next().unwrap();
        cost = std::cmp::max(cost, c);
        st.remove(&(c, c_self, i));
        for j in graph[i].clone() {
            let x = st_save[j];
            st.remove(&x);
            let y = (x.0 - (-c_self) as usize, x.1, x.2);
            graph[j].remove(&i);
            st.insert(y);
            st_save[j] = y;
        }
    }

    println!("{}", cost);
}
