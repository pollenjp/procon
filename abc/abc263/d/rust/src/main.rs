use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_l: i64,
        in_r: i64,
        in_a: [i64; in_n],
    }

    let a_sum = in_a.iter().sum::<i64>();

    let mut l_vec = vec![0; in_n];
    let mut r_vec = vec![0; in_n];

    for i in 0..in_n {
        l_vec[i] = in_l as i64 - in_a[i];
        if i > 0 {
            l_vec[i] += l_vec[i - 1];
        }
    }

    for i_rev in (0..in_n).rev() {
        r_vec[i_rev] = in_r as i64 - in_a[i_rev];
        if i_rev < in_n - 1 {
            r_vec[i_rev] += r_vec[i_rev + 1];
        }
    }

    let mut r_mins = r_vec.clone();

    for i_rev in (0..in_n).rev() {
        if i_rev == in_n - 1 {
            r_mins[i_rev] = std::cmp::min(0, r_vec[i_rev]);
        } else {
            r_mins[i_rev] = std::cmp::min(r_vec[i_rev], r_mins[i_rev + 1]);
        }
    }

    // dbg!(&l_vec);
    // dbg!(&r_vec);
    // dbg!(&r_mins);

    let mut min_diff = r_mins[0];
    min_diff = std::cmp::min(min_diff, l_vec[in_n - 1]);
    for i in 0..in_n - 1 {
        min_diff = std::cmp::min(min_diff, l_vec[i] + r_mins[i + 1]);
    }

    if min_diff < 0 {
        println!("{}", a_sum + min_diff);
    } else {
        println!("{}", a_sum);
    }
}
