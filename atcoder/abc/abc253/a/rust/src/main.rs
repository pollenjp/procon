fn main() {
    proconio::input! {
        in_abc: [u32; 3],
    }

    let mut abc = in_abc.clone();

    abc.sort();

    if abc[1] == in_abc[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
