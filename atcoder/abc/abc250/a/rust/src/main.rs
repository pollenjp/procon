use std::cmp::Ordering;
use Ordering::*;

fn main() {
    proconio::input! {
        in_h: usize,
        in_w: usize,
        in_r: usize,
        in_c: usize,
    }

    dbg!(&in_h, &in_w, &in_r, &in_c);

    let mut count: usize = 0;
    match (in_r.cmp(&1), in_r.cmp(&in_h)) {
        (Equal, Equal) => {}
        (Equal, Less) | (Greater, Equal) => {
            count += 1;
        }
        _ => {
            count += 2;
        }
    }
    dbg!(&count);

    match (in_c.cmp(&1), in_c.cmp(&in_w)) {
        (Equal, Equal) => {}
        (Equal, Less) | (Greater, Equal) => {
            count += 1;
        }
        _ => {
            count += 2;
        }
    }
    dbg!(&count);

    println!("{}", count);
}
