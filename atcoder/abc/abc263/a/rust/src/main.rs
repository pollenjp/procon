use proconio::input;

fn main() {
    input! {
        in_vec: [usize; 5],
    }

    let mut cnt_vec = vec![0; 14];

    for v in in_vec {
        cnt_vec[v] += 1;
    }

    let mut flag = false;
    for &cnt in cnt_vec.iter() {
        if cnt == 3 {
            flag = true;
        }
    }

    if !flag {
        println!("No");
        return;
    }

    let mut flag = false;
    for &cnt in cnt_vec.iter() {
        if cnt == 2 {
            flag = true;
        }
    }

    if !flag {
        println!("No");
        return;
    }
    println!("Yes");
}
