use proconio::input;
fn main() {
    input! {
        _n: usize,
        in_k: usize,
        in_s: String,
    }

    let alphabets = "abcdefghijklmnopqrstuvwxyz";

    let mut tgt_chars = vec![];

    let mut search_start_idx = 0;

    let mut target_current_idx = 0;

    while target_current_idx < in_k {
        for alphabet in alphabets.chars() {
            let idx_offset = in_s[search_start_idx..in_s.len() - (in_k - (target_current_idx + 1))]
                .find(alphabet)
                .unwrap_or(in_s.len());
            let idx = search_start_idx + idx_offset;
            // dbg!(&search_start_idx, &alphabet, &idx_offset);
            if idx >= in_s.len() {
                // not found
                continue;
            }

            search_start_idx = idx + 1;
            target_current_idx += 1;
            tgt_chars.push(alphabet);
            break;
        }
    }

    println!("{}", tgt_chars.iter().collect::<String>());
}
