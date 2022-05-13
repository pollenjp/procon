#![warn(clippy::pedantic, clippy::nursery)]

fn main() {
    proconio::input! {
        in_n: usize,
        in_k: usize,
        s_vec: [String ; in_n],
    }

    // bit full search
    let mut max_num_of_target = 0;
    for bit in 0..(1 << in_n) {
        let sub_list = (0..in_n)
            .filter(|x| (bit & (1 << x)) != 0)
            .collect::<Vec<_>>();
        // dbg!(sub_list);

        if sub_list.len() < in_k {
            continue;
        }

        let mut alphabet_count = [0; 26];
        for i in &sub_list {
            let s = &s_vec[*i];
            for c in s.chars() {
                alphabet_count[(c as u32 - 'a' as u32) as usize] += 1;
            }
        }

        let mut num_of_target = 0;
        for count in alphabet_count.iter() {
            if count == &in_k {
                num_of_target += 1;
            }
        }

        max_num_of_target = std::cmp::max(max_num_of_target, num_of_target);
    }

    println!("{}", max_num_of_target);
}
