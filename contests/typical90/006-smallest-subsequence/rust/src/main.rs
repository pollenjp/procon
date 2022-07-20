#![feature(map_first_last)]
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        _n: usize,
        in_k: usize,
        in_s: String,
    }

    let mut mp = BTreeMap::new();

    let mut mp_start_idx: usize = 0;
    let mut mp_next_start_idx: usize = 0;
    let mp_end_idx: usize = in_s.len()-in_k+1;
    let mut tgt_chars: Vec<char> = Vec::new();

    for k in 0..in_k {
        // k番目のcharを選択

        for i in mp_next_start_idx..mp_end_idx+k {
            let c = in_s.chars().nth(i).unwrap();
            if mp.contains_key(&c) {
                mp.insert(c, mp[&c]+1);
            } else {
                mp.insert(c, 1);
            }
        }
        mp_next_start_idx = mp_end_idx+k;

        let (&tgt_c, _) = mp.first_key_value().unwrap();
        tgt_chars.push(tgt_c);

        for i in mp_start_idx..mp_end_idx+k {
            let c = in_s.chars().nth(i).unwrap();
            if mp.contains_key(&c) {
                mp.insert(c, mp[&c]-1);
                if mp[&c] == 0 {
                    mp.remove(&c);
                }
            } else {
                panic!("{} is not contained!", c);
            }

            if c == tgt_c {
                mp_start_idx = i+1;
                break;
            }
        }
    }

    println!("{}", tgt_chars.iter().collect::<String>());
}
