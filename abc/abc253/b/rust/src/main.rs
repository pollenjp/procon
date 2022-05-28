fn main() {
    proconio::input! {
        in_h: usize,
        _in_w: usize,
        s_vec: [String; in_h],
    }

    let mut target_pair: Vec<(i32, i32)> = vec![];

    for it in s_vec.iter().enumerate() {
        let (h, s) = it;
        for it2 in s.chars().enumerate() {
            let (w, c) = it2;
            if c != 'o' {
                continue;
            }

            target_pair.push((h.clone() as i32, w.clone() as i32));
        }
    }

    let (h1, w1) = target_pair[0];
    let (h2, w2) = target_pair[1];

    let mut ans = 0;
    ans += (h1 - h2).abs();
    ans += (w1 - w2).abs();

    println!("{}", ans);
}
