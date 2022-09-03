use proconio::input;

fn main() {
    input! {
        in_n: usize,
        mut in_a_vec: [i64; in_n],
        mut in_b_vec: [i64; in_n],
    }

    in_a_vec.sort();
    in_b_vec.sort();

    let mut cost = 0;
    for i in 0..in_n {
        cost += (in_a_vec[i] - in_b_vec[i]).abs();
    }
    println!("{}", cost);
}
