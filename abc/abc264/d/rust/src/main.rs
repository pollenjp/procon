use proconio::input;

static TARGET: &str = "atcoder";

fn main() {
    input! {
        in_s: String
    }

    let mut ans_cnt = 0;

    let mut in_s = in_s;

    // let mut next_idx: usize = 0;
    for target_idx in 0..TARGET.len() {
        let target_ch = TARGET.chars().nth(target_idx).unwrap();
        let mut current_idx = in_s.chars().position(|ch| ch == target_ch).unwrap();
        while current_idx != target_idx {
            let tmp1 = in_s.chars().nth(current_idx).unwrap();
            let tmp2 = in_s.chars().nth(current_idx - 1).unwrap();
            in_s.replace_range(current_idx..current_idx + 1, &tmp2.to_string());
            in_s.replace_range(current_idx - 1..current_idx, &tmp1.to_string());
            ans_cnt += 1;
            current_idx -= 1;
        }
    }

    println!("{}", ans_cnt);
}
