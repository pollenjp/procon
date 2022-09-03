use proconio::input;
use std::collections::BTreeSet;

static NUM_S: usize = 2 * 100_000;

fn main() {
    input! {
        in_n: usize,
        in_q: usize,
        in_ab: [(usize, usize); in_n],
        in_cd: [(usize, usize); in_q],
    };

    let in_ab = in_ab.iter().map(|&(a, b)| (a, b - 1)).collect::<Vec<_>>();
    let in_cd = in_cd
        .iter()
        .map(|&(c, d)| (c - 1, d - 1))
        .collect::<Vec<_>>();

    // rate, kids id
    let mut ans_st: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut each_kg_st: Vec<BTreeSet<(usize, usize)>> = vec![BTreeSet::new(); NUM_S];
    let mut each_kid2kg = vec![0; in_n];

    for (i, &(a, b)) in in_ab.iter().enumerate() {
        let val = (a, i);
        each_kg_st[b].insert(val);
        each_kid2kg[i] = b;
    }

    for i in 0..NUM_S {
        let result = each_kg_st[i].iter().next_back();
        match result {
            None => {
                continue;
            }
            Some(&val) => if !ans_st.insert(val) {},
        }
    }
    // query
    for &(kid_id, kg) in &in_cd {
        let kg_pre = each_kid2kg[kid_id];
        each_kid2kg[kid_id] = kg;

        let kid_score = in_ab[kid_id].0;
        let st_val = (kid_score, kid_id);

        each_kg_st[kg_pre].remove(&st_val);
        ans_st.remove(&st_val);
        match each_kg_st[kg_pre].iter().next_back() {
            Some(&val) => if ans_st.insert(val) {},
            None => (),
        }

        match each_kg_st[kg].iter().next_back() {
            Some(&val) => if ans_st.remove(&val) {},
            None => (),
        }

        each_kg_st[kg].insert(st_val);

        match each_kg_st[kg].iter().next_back() {
            Some(&val) => if ans_st.insert(val) {},
            None => (),
        }

        match ans_st.iter().next() {
            None => {}
            Some(&(score, _kid_id)) => {
                println!("{}", score);
            }
        }
    }
}
