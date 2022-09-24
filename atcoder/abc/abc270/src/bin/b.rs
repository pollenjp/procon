use proconio::input;
fn main() {
    input! {
        in_x: i32,
        in_y: i32,
        in_z: i32,
    }

    let x;
    let y;
    let z;

    if in_x > 0 {
        x = in_x;
        y = in_y;
        z = in_z;
    } else {
        x = -in_x;
        y = -in_y;
        z = -in_z;
    }

    if x > y && y > 0 {
        // ゴールとの間に y があるときは z を経由する必要がある.
        if z > y {
            // z に到達不可能
            println!("-1");
            return;
        }

        let mut ans = 0;
        ans += z.abs();
        ans += (x - z).abs();
        println!("{}", ans);
        return;
    } else {
        println!("{}", x);
        return;
    }
}
