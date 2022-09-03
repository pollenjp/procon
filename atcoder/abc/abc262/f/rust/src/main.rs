use proconio::input;
use std::collections::VecDeque;

// remain_op: 残り可能操作回数
fn select<T>(val_cost_vec: &VecDeque<(T, T)>, remain_op: T) -> Vec<T>
where
    T: std::cmp::Ord
        + std::cmp::PartialOrd
        + std::cmp::PartialEq
        + std::cmp::Eq
        + std::ops::Add
        + std::ops::AddAssign
        + std::ops::Sub
        + std::ops::SubAssign
        + Copy
        + std::fmt::Debug
        + Default,
{
    let mut remain_op = remain_op;
    let n = val_cost_vec.len();
    let mut stack: VecDeque<(T, T)> = VecDeque::new();
    for i in 0..=n {
        let cur_val_cost;
        if i < n {
            cur_val_cost = val_cost_vec[i].clone();
        } else {
            cur_val_cost = (T::default(), T::default());
        }
        while !stack.is_empty()
            && stack.back().unwrap().1 <= remain_op
            && cur_val_cost < *stack.back().unwrap()
        {
            // 削除操作を行う
            remain_op -= stack.back().unwrap().1;
            stack.pop_back();
        }
        stack.push_back(cur_val_cost);
    }
    stack.pop_back();

    let mut ret = vec![T::default(); stack.len()];
    for i in 0..stack.len() {
        ret[i] = stack[i].0;
    }
    ret
}

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
        in_p: [usize; in_n]
    }

    let mut p_cost_vec: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..in_n {
        p_cost_vec.push_back((in_p[i], 1));
    }

    let mut ans = select(&p_cost_vec, in_k as usize);

    if in_k != 0 {
        let back_min = *in_p[in_n - in_k..in_n].iter().min().unwrap();
        let mut back_min_idx = 0;
        for i in in_n - in_k..in_n {
            if in_p[i] == back_min {
                back_min_idx = i;
                break;
            }
        }

        let mut p_cost_rotated = p_cost_vec.clone();
        let mut remain_op = in_k;
        for _ in back_min_idx..in_n {
            let mut back = p_cost_rotated.pop_back().unwrap();
            back.1 = 0; // cost 0
            p_cost_rotated.push_front(back);
            remain_op -= 1;
        }

        ans = std::cmp::min(ans, select(&p_cost_rotated, remain_op as usize));
    }

    for i in 0..ans.len() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
}
