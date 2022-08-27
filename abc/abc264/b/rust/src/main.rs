use proconio::input;

static GRID: [[bool; 15]; 15] = [
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true,
    ],
    [
        true, false, true, true, true, true, true, true, true, true, true, true, true, false, true,
    ],
    [
        true, false, true, false, false, false, false, false, false, false, false, false, true,
        false, true,
    ],
    [
        true, false, true, false, true, true, true, true, true, true, true, false, true, false,
        true,
    ],
    [
        true, false, true, false, true, false, false, false, false, false, true, false, true,
        false, true,
    ],
    [
        true, false, true, false, true, false, true, true, true, false, true, false, true, false,
        true,
    ],
    [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true,
    ],
    [
        true, false, true, false, true, false, true, true, true, false, true, false, true, false,
        true,
    ],
    [
        true, false, true, false, true, false, false, false, false, false, true, false, true,
        false, true,
    ],
    [
        true, false, true, false, true, true, true, true, true, true, true, false, true, false,
        true,
    ],
    [
        true, false, true, false, false, false, false, false, false, false, false, false, true,
        false, true,
    ],
    [
        true, false, true, true, true, true, true, true, true, true, true, true, true, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ],
];

fn main() {
    input! {
        in_r: usize,
        in_c: usize,
    }
    let r = in_r - 1;
    let c = in_c - 1;
    if GRID[r][c] {
        println!("{}", "black");
    } else {
        println!("{}", "white");
    }
}
