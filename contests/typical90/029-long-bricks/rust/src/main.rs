use proconio::input;

#[derive(Debug)]
struct SegmentTree {
    n: usize,
    seg: Vec<i64>,
    lazy: Vec<i64>,
    id_element: i64, // 単位元
}

impl SegmentTree {
    fn new(size: usize) -> Self {
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
            id_element: 0,
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

fn main() {
    input! {
        in_w: usize,
        in_n: usize,
        in_lr: [(usize, usize); in_n],
    }

    let mut seg = SegmentTree::new(in_w);

    for &(l, r) in &in_lr {
        let height = seg.query(l - 1, r) + 1;
        // dbg!(l, r, height);
        // dbg!(&seg.seg, &seg.lazy);
        seg.update(l - 1, r, height);
        // dbg!(&seg);
        println!("{}", height);
    }
}
