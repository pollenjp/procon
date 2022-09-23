use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
        in_a: [usize; in_n],
    }

    // 尺取り法
    let mut accum = vec![0; in_a.len() + 1];
    for i in 0..in_a.len() {
        accum[i + 1] = accum[i] + in_a[i];
    }

    let mut cnt = 0;

    let mut right = 0;
    for left in 0..accum.len() - 1 {
        right = right.max(left + 1);

        for r in right..accum.len() {
            if accum[r] - accum[left] >= in_k {
                right = r;
                break;
            }
        }

        if accum[right] - accum[left] < in_k {
            continue;
        }

        cnt += accum.len() - right;
    }

    println!("{}", cnt);
}
