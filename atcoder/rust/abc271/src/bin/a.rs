use proconio::input;

fn main() {
    input! {
        in_n: usize,
    }

    let arr = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut x = 0;
    let mut ans = String::new();
    let y = in_n >> 4;
    for i in 0..4 {
        x += y & (1 << i);
        dbg!(&x);
    }
    ans.push(arr[x]);

    let mut x = 0;
    let y = in_n;
    for i in 0..4 {
        x += y & (1 << i);
    }
    ans.push(arr[x]);

    println!("{}", ans);
}
