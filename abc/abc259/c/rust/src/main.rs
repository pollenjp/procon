use proconio::input;

fn create_char_cnt_vec(s: &str) -> Vec<(char, usize)> {
    let mut cnt_vec = Vec::new();
    let mut prev_c = ' ';
    let mut cnt_char = 0;
    for c in s.chars() {
        if prev_c == ' ' {
            prev_c = c;
            cnt_char = 1;
            continue;
        }

        if c == prev_c {
            cnt_char += 1;
        } else {
            cnt_vec.push((prev_c, cnt_char));
            prev_c = c;
            cnt_char = 1;
        }
    }

    if prev_c != ' ' {
        cnt_vec.push((prev_c, cnt_char));
    }
    cnt_vec
}

fn main() {
    input! {
        in_s: String,
        in_t: String,
    }

    let s_char_cnt_vec = create_char_cnt_vec(&in_s);
    let t_char_cnt_vec = create_char_cnt_vec(&in_t);

    if s_char_cnt_vec.len() != t_char_cnt_vec.len() {
        println!("No");
        return;
    }

    // dbg!(&s_char_cnt_vec);
    // dbg!(&t_char_cnt_vec);

    for i in 0..s_char_cnt_vec.len() {
        let s_char_cnt = s_char_cnt_vec[i];
        let t_char_cnt = t_char_cnt_vec[i];
        if s_char_cnt.0 != t_char_cnt.0 {
            println!("No");
            return;
        }

        if s_char_cnt.1 == t_char_cnt.1 {
            continue;
        }

        if 2 <= s_char_cnt.1 && s_char_cnt.1 < t_char_cnt.1 {
            continue;
        }

        println!("No");
        return;
    }

    println!("Yes");
}
