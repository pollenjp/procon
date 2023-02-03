use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_l: usize,
        in_k: usize,
        in_a: [u32; in_n],
    }

    // dbg!(&in_n, &in_l, &in_k, &in_a);

    let mut ys = vec![0];
    ys.extend(in_a);
    ys.push(in_l as u32);
    // dbg!(&ys);

    let mut ok = 0 as u32;
    let mut ng = 1e9 as u32;
    let mut x;
    while (ng - ok) > 1 {
        x = (ng - ok) / 2 + ok;
        let mut k0 = 0;
        let mut k1 = 0;
        let mut cnt = 0;
        loop {
            if k1 >= ys.len() {
                ng = x;
                break;
            }

            if ys[k1] - ys[k0] < x {
                k1 += 1;
                continue;
            }

            k0 = k1;
            k1 = k0 + 1;
            cnt += 1;
            if cnt > in_k {
                ok = x;
                break;
            }
        }
    }

    // dbg!(&ok, &ng);
    println!("{}", ok);
}
