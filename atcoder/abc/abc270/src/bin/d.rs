use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
        in_a: [usize; in_k],
    }

    let mut a_arr = in_a.clone();
    // a_arr.sort();
    a_arr.reverse();

    let mut ans = 0;
    let mut num_stones = in_n;
    let mut turn: usize = 0;

    for &a in a_arr.iter() {
        let b = 2 * a;

        let q = num_stones / b;
        ans += q * a;

        let mut r = num_stones % b;
        if r >= a {
            if turn == 0 {
                ans += a;
            }
            turn = (turn + 1) % 2;
            r -= a;
        }
        num_stones = r;
        if num_stones == 0 {
            break;
        }
    }
    println!("{}", ans);
}
