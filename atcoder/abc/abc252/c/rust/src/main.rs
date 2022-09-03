fn main() {
    proconio::input! {
        n: usize,
        s_vec: [String; n],
    }

    let mut time_vec = vec![0; 10];

    for target_num in 0..=9 {
        // time

        let mut time = 0;

        let mut char_cnt = 0;

        for i in 0..s_vec[0].len() {
            let mut current_target_char_cnt = 0;
            for s in s_vec.iter() {
                let c = s.chars().nth(i).unwrap();
                if c != std::char::from_digit(target_num as u32, 10).unwrap() {
                    continue;
                }
                current_target_char_cnt += 1;
            }

            // check max time
            char_cnt += current_target_char_cnt;
            if current_target_char_cnt > 0 {
                if target_num == 0 {
                    time = time.max(10 * (current_target_char_cnt - 1) + i);
                } else {
                    time = time.max(10 * (current_target_char_cnt - 1) + i);
                }
            }

            if char_cnt >= n {
                break;
            }
        }
        time_vec[target_num] = time;
    }

    dbg!(&time_vec);

    println!("{}", time_vec.iter().min().unwrap());
}
