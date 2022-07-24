use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
    }

    let mut prime_cnt = vec![0; in_n + 1];

    for p in 2..=in_n {
        if prime_cnt[p] > 0 {
            continue;
        }

        let mut x = p;
        while x <= in_n {
            prime_cnt[x] += 1;
            x += p;
        }
    }

    let total = prime_cnt.iter().filter(|&&c| c >= in_k).count();
    println!("{}", total);
}
