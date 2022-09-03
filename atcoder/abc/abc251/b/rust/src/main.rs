#![warn(clippy::pedantic, clippy::nursery)]
use itertools::Itertools;

fn main() {
    proconio::input! {
        in_n: usize,
        in_w: usize,
        in_a_vec: [u32; in_n],
    }

    let mut is_good_int_vec: Vec<bool> = vec![false; in_w + 1];
    // dbg!(&in_a_vec);

    for i in 1..=3 {
        for x_comb in in_a_vec.iter().combinations(i) {
            let sum = x_comb.into_iter().sum::<u32>();
            if sum <= in_w as u32 {
                is_good_int_vec[sum as usize] = true;
            }
            // dbg!(sum);
        }
    }
    // dbg!(&is_good_int_vec);

    let mut count = 0;
    for is_good_int in is_good_int_vec.iter() {
        if *is_good_int {
            count += 1;
        }
    }
    println!("{}", count);
}
