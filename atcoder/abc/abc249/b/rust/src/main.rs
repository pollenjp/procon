#![warn(clippy::pedantic, clippy::nursery)]
use std::collections::HashSet;

fn main() {
    proconio::input! {
        in_s: String,
    }

    let mut char_set = HashSet::new();
    let mut include_lowercase = false;
    let mut include_uppercase = false;
    let mut duplication = false;
    for c in in_s.chars() {
        if char_set.contains(&c) {
            duplication = true;
            break;
        }
        char_set.insert(c);

        if !include_lowercase && c.is_ascii_lowercase() {
            include_lowercase = true;
        }
        if !include_uppercase && !c.is_ascii_lowercase() {
            include_uppercase = true;
        }
    }

    if !duplication && include_lowercase && include_uppercase {
        println!("Yes");
        return;
    }
    println!("No");
}
