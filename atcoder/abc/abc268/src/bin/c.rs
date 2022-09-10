use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_p: [usize; in_n],
    };

    let mut ans = vec![0; in_n];
    for i in 0..in_n {
        // in_p[i] が 人を喜ばすのは何回転の時か
        // x >= 1
        let x = (in_p[i] + in_n) - i;
        ans[x % in_n] += 1;
        ans[(x - 1) % in_n] += 1;
        ans[(x + 1) % in_n] += 1;
    }

    println!("{}", ans.iter().max().unwrap());
}
