use proconio::input;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RangeMaxQuery {
    // the number of the segment-tree's bottom elements
    n: usize,
    // segment tree
    seg: Vec<i32>,
    // 遅延評価用
    lazy: Vec<i32>,
    lazy_need_update: Vec<bool>,
}

impl RangeMaxQuery {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        RangeMaxQuery {
            n,
            seg: vec![std::i32::MIN; n * 2 - 1],
            lazy: vec![std::i32::MIN; n * 2 - 1],
            lazy_need_update: vec![false; n * 2 - 1],
        }
    }

    fn eval(&mut self, k: usize) {
        if !self.lazy_need_update[k] {
            return;
        }

        if k < self.n - 1 {
            self.lazy_update(k * 2 + 1, self.lazy[k].clone(), true);
            self.lazy_update(k * 2 + 2, self.lazy[k].clone(), true);
        }

        self.seg[k] = self.lazy[k];
        self.lazy_update(k, std::i32::MIN, false);
    }

    // [a, b) の query を求める
    // [l, r) は segment している範囲
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i32 {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return std::i32::MIN;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            return self.seg[k];
        }

        let vl = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
        std::cmp::max(vl, vr)
    }

    // [a, b) の query を求める
    fn query(&mut self, a: usize, b: usize) -> i32 {
        self.query_sub(a, b, 0, 0, self.n)
    }

    fn lazy_update(&mut self, k: usize, x: i32, need_update: bool) {
        self.lazy[k] = x;
        self.lazy_need_update[k] = need_update;
    }

    // [a, b) かつ [l, r) の範囲の値を x で update する
    fn update_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, x: i32) {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            self.lazy_update(k, x, true);
            self.eval(k);
            return;
        }

        self.update_sub(a, b, k * 2 + 1, l, (l + r) / 2, x);
        self.update_sub(a, b, k * 2 + 2, (l + r) / 2, r, x);
        self.seg[k] = std::cmp::max(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
    }

    // [a, b) の範囲の値を x で update する
    fn update(&mut self, a: usize, b: usize, x: i32) {
        self.update_sub(a, b, 0, 0, self.n, x);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RangeMinQuery {
    // the number of the segment-tree's bottom elements
    n: usize,
    // segment tree
    seg: Vec<i32>,
    // 遅延評価用
    lazy: Vec<i32>,
    lazy_need_update: Vec<bool>,
}

impl RangeMinQuery {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        RangeMinQuery {
            n,
            seg: vec![std::i32::MAX; n * 2 - 1],
            lazy: vec![std::i32::MAX; n * 2 - 1],
            lazy_need_update: vec![false; n * 2 - 1],
        }
    }

    fn eval(&mut self, k: usize) {
        if !self.lazy_need_update[k] {
            return;
        }

        if k < self.n - 1 {
            self.lazy_update(k * 2 + 1, self.lazy[k].clone(), true);
            self.lazy_update(k * 2 + 2, self.lazy[k].clone(), true);
        }

        self.seg[k] = self.lazy[k];
        self.lazy_update(k, std::i32::MAX, false);
    }

    // [a, b) の query を求める
    // [l, r) は segment している範囲
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i32 {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return std::i32::MAX;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            return self.seg[k];
        }

        let vl = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
        std::cmp::min(vl, vr)
    }

    // [a, b) の query を求める
    fn query(&mut self, a: usize, b: usize) -> i32 {
        self.query_sub(a, b, 0, 0, self.n)
    }

    fn lazy_update(&mut self, k: usize, x: i32, need_update: bool) {
        self.lazy[k] = x;
        self.lazy_need_update[k] = need_update;
    }

    // [a, b) かつ [l, r) の範囲の値を x で update する
    fn update_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, x: i32) {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            self.lazy_update(k, x, true);
            self.eval(k);
            return;
        }

        self.update_sub(a, b, k * 2 + 1, l, (l + r) / 2, x);
        self.update_sub(a, b, k * 2 + 2, (l + r) / 2, r, x);
        self.seg[k] = std::cmp::min(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
    }

    // [a, b) の範囲の値を x で update する
    fn update(&mut self, a: usize, b: usize, x: i32) {
        self.update_sub(a, b, 0, 0, self.n, x);
    }
}
static NUM_S: usize = 2 * 100_000;

fn main() {
    input! {
        in_n: usize,
        in_q: usize,
        in_ab: [(usize, usize); in_n],
        in_cd: [(usize, usize); in_q],
    };

    let in_ab = in_ab
        .into_iter()
        .map(|(a, b)| (a, b - 1))
        .collect::<Vec<_>>();
    let in_cd = in_cd
        .into_iter()
        .map(|(c, d)| (c - 1, d - 1))
        .collect::<Vec<_>>();

    let mut ans_ruq = RangeMinQuery::new(NUM_S);
    // kd: kindergarten
    let mut kids2kg = in_ab.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    let mut kg_each_ruq = vec![RangeMaxQuery::new(in_n); NUM_S];

    let mut kg_each_num = vec![0; NUM_S];

    // initialize
    for (i, (a, b)) in in_ab.iter().enumerate() {
        kg_each_ruq[*b].update(i, i + 1, *a as i32);
        kg_each_num[*b] += 1;
        let size = kg_each_ruq[*b].n;
        let val = kg_each_ruq[*b].query(0, size);
        if val == std::i32::MIN {
            ans_ruq.update(*b, *b + 1, std::i32::MAX);
        } else {
            ans_ruq.update(*b, *b + 1, val);
        }
    }

    for &(kids_idx, c_next_kg) in in_cd.iter() {
        let c_current_kg = kids2kg[kids_idx];

        // erase old data
        kg_each_ruq[c_current_kg].update(kids_idx, kids_idx + 1, std::i32::MIN);
        // update each rate
        kg_each_ruq[c_next_kg].update(kids_idx, kids_idx + 1, in_ab[kids_idx].0 as i32);

        kids2kg[kids_idx] = c_next_kg;

        // update kids number
        kg_each_num[c_current_kg] -= 1;
        kg_each_num[c_next_kg] += 1;

        // update ans_ruq
        let mut val = kg_each_ruq[c_current_kg].query(0, in_n);
        if val == std::i32::MIN {
            val = std::i32::MAX;
        }
        ans_ruq.update(c_current_kg, c_current_kg + 1, val);

        let val = kg_each_ruq[c_next_kg].query(0, in_n);
        ans_ruq.update(c_next_kg, c_next_kg + 1, val);

        println!("{}", ans_ruq.query(0, NUM_S));
    }
}
