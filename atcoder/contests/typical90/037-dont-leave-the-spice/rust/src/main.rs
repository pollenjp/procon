use proconio::input;
#[derive(Debug)]
struct SegmentTree {
    n: usize,
    seg: Vec<i64>,
    lazy: Vec<i64>,
    id_element: i64, // 単位元
}

impl SegmentTree {
    fn new(size: usize, id_element: i64) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        let seg = vec![0; n * 2 - 1];
        let lazy = vec![0; n * 2 - 1];
        SegmentTree {
            n,
            seg,
            lazy,
            id_element: id_element,
        }
    }

    fn eval(&mut self, k: usize) {
        if self.lazy[k] == self.id_element {
            return;
        }

        if k < self.n - 1 {
            self.lazy[k * 2 + 1] = self.lazy[k];
            self.lazy[k * 2 + 2] = self.lazy[k];
        }

        self.seg[k] = self.lazy[k];
        self.lazy[k] = self.id_element;
    }

    // [a, b) の query を求める
    // [l, r) は segment している範囲
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return self.id_element;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            return self.seg[k];
        }

        let vl = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
        // dbg!(vl, vr);
        std::cmp::max(vl, vr)
    }

    // [a, b) の query を求める
    fn query(&mut self, a: usize, b: usize) -> i64 {
        self.query_sub(a, b, 0, 0, self.n)
    }

    fn update_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, x: i64) {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            self.lazy[k] = x;
            self.eval(k);
            return;
        }

        self.update_sub(a, b, k * 2 + 1, l, (l + r) / 2, x);
        self.update_sub(a, b, k * 2 + 2, (l + r) / 2, r, x);
        self.seg[k] = std::cmp::max(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
    }

    fn update(&mut self, a: usize, b: usize, x: i64) {
        self.update_sub(a, b, 0, 0, self.n, x);
    }
}

fn print_table(table: &Vec<Vec<i64>>) {
    let row_size = table.len();
    if row_size == 0 {
        return;
    }
    let col_size = table[0].len();
    let mut col_size_vec = vec![0; col_size];
    for row in table.iter() {
        for (c_idx, cell) in row.iter().enumerate() {
            let s = format!("{}", cell);
            col_size_vec[c_idx] = std::cmp::max(col_size_vec[c_idx], s.len());
        }
    }
    for row in table.iter() {
        for (c_idx, cell) in row.iter().enumerate() {
            print!("{:>width$} ", cell, width = col_size_vec[c_idx] + 1);
        }
        println!();
    }
}

fn main() {
    input! {
        in_w: usize,
        in_n: usize,
        in_lrv: [(usize, usize, usize); in_n],
    };

    // i (ax0-index) 番目の対象物を選択する際に重みが合計でちょうど j (ax1-index) になるときの最大価値
    // -1: 不可能
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; in_w + 1]; in_n + 1];
    dp[0][0] = 0;
    let mut pre_seg_tree = SegmentTree::new(in_n + 1, std::i64::MIN);
    pre_seg_tree.update(1, in_w + 1, -1);
    for i in 1..in_n + 1 {
        let (l, r, v) = in_lrv[i - 1];

        for j in 0..=in_w {
            // case: not select i-th item
            dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j]);
            // case: select i-th item
            let k_l = std::cmp::max(j as i32 - r as i32, 0) as usize;
            let k_r = std::cmp::min(std::cmp::max(j as i32 - l as i32 + 1, 0) as usize, in_w + 1);
            let pre_val = pre_seg_tree.query(k_l, k_r);
            if pre_val > -1 {
                dp[i][j] = std::cmp::max(dp[i][j], pre_val + v as i64);
            }
        }

        pre_seg_tree = SegmentTree::new(in_w + 1, std::i64::MIN);
        for j in 0..in_w + 1 {
            pre_seg_tree.update(j, j + 1, dp[i][j] as i64);
        }
    }

    // print_table(&dp);

    let v = dp[in_n][in_w];
    if v > 0 {
        println!("{}", v);
    } else {
        println!("-1");
    }
}
