fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a_vec: [i32; n],
        b_vec: [i32; k],
    }

    let mut most_delicious_idxes: Vec<usize> = vec![];
    let max_delicious_val = a_vec.iter().max().unwrap();
    for iter in a_vec.iter().enumerate() {
        if iter.1 == max_delicious_val {
            most_delicious_idxes.push(iter.0 as usize);
        }
    }

    let mut is_disliked = vec![false; n];
    for b in b_vec.iter() {
        is_disliked[*b as usize - 1] = true;
    }

    // dbg!(&is_disliked);
    // dbg!(&most_delicious_idxes);
    for i in most_delicious_idxes.iter() {
        if is_disliked[*i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
    return;
}
