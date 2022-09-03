use proconio::input;

trait Value<T>
where
    T: Copy + std::fmt::Display,
{
    fn value(&self) -> T;
}

impl<T> std::fmt::Display for dyn Value<T>
where
    T: Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

trait Monoid<T>: Value<T>
where
    T: Copy + std::fmt::Display,
{
    fn identity() -> Self;
    fn op(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SegmentTree<T, MonoidT>
where
    T: Copy + std::fmt::Display,
    MonoidT: Monoid<T> + Value<T> + Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    // the number of the segment-tree's bottom elements
    n: usize,
    // segment tree
    seg: Vec<MonoidT>,
    // 遅延評価用
    lazy: Vec<MonoidT>,
    lazy_need_update: Vec<bool>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, MonoidT> SegmentTree<T, MonoidT>
where
    T: Copy + std::fmt::Display,
    MonoidT: Monoid<T> + Value<T> + Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        SegmentTree {
            n,
            seg: vec![MonoidT::identity(); n * 2 - 1],
            lazy: vec![MonoidT::identity(); n * 2 - 1],
            lazy_need_update: vec![false; n * 2 - 1],
            _phantom: std::marker::PhantomData,
        }
    }

    fn eval(&mut self, k: usize) {
        if !self.lazy_need_update[k] {
            return;
        }

        if k < self.n - 1 {
            self.lazy_update(k * 2 + 1, &self.lazy[k].clone(), true);
            self.lazy_update(k * 2 + 2, &self.lazy[k].clone(), true);
        }

        self.seg[k] = self.lazy[k];
        self.lazy_update(k, &MonoidT::identity().clone(), false);
    }

    // [a, b) の query を求める
    // [l, r) は segment している範囲
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> MonoidT {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return MonoidT::identity();
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            return self.seg[k];
        }

        let vl = self.query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
        vl.op(&vr)
    }

    // [a, b) の query を求める
    fn query(&mut self, a: usize, b: usize) -> MonoidT {
        self.query_sub(a, b, 0, 0, self.n)
    }

    fn lazy_update(&mut self, k: usize, x: &MonoidT, need_update: bool) {
        self.lazy[k] = *x;
        self.lazy_need_update[k] = need_update;
    }

    // [a, b) かつ [l, r) の範囲の値を x で update する
    fn update_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize, x: MonoidT) {
        self.eval(k);

        if r <= a || b <= l {
            // out of range
            return;
        }

        if a <= l && r <= b {
            // segment [l, r) が [a, b) に入っている
            self.lazy_update(k, &x, true);
            self.eval(k);
            return;
        }

        self.update_sub(a, b, k * 2 + 1, l, (l + r) / 2, x);
        self.update_sub(a, b, k * 2 + 2, (l + r) / 2, r, x);
        self.seg[k] = self.seg[k * 2 + 1].op(&self.seg[k * 2 + 2]);
    }

    // [a, b) の範囲の値を x で update する
    fn update(&mut self, a: usize, b: usize, x: MonoidT) {
        self.update_sub(a, b, 0, 0, self.n, x);
    }
}

fn padding_table<T>(table: &Vec<Vec<T>>) -> Vec<Vec<String>>
where
    T: std::fmt::Display,
{
    let row_num = table.len();
    if row_num == 0 {
        return vec![];
    }
    let col_num = table.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut col_size_vec = vec![0; col_num];
    for row_idx in 0..table.len() {
        for col_idx in 0..table[row_idx].len() {
            col_size_vec[col_idx] = std::cmp::max(
                col_size_vec[col_idx],
                format!("{}", &table[row_idx][col_idx]).len(),
            );
        }
    }

    let mut ret = vec![vec![String::new(); col_num]; row_num];

    for row_idx in 0..table.len() {
        for col_idx in 0..table[row_idx].len() {
            let col_size = col_size_vec[col_idx];
            ret[row_idx][col_idx] =
                format!("{:>width$}", &table[row_idx][col_idx], width = col_size);
        }
    }

    ret
}

impl<T, MonoidT> std::fmt::Display for SegmentTree<T, MonoidT>
where
    T: Copy + std::fmt::Display,
    MonoidT: Monoid<T> + Value<T> + Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tree_level: usize = 1;
        let elem_total_num = self.seg.len();
        let mut n: usize = 1;
        while n < elem_total_num {
            n <<= 1;
            tree_level += 1;
        }
        if n != elem_total_num + 1 {
            panic!("n != elem_total_num + 1: {}, {}", n, elem_total_num);
        }

        let mut display_vals = vec![vec![String::new(); self.n]; tree_level];

        //
        // n: elem_all_num <- その階層までに見えている要素の数
        // l: level_idx
        // elem_index:その階層の要素のインデックス
        // elem_num: その階層の要素数
        //    n |   l |       i       | elem_num |
        // -----|-----|---------------|----------|
        //    1 |   0 |       0       |        1 |
        //    3 |   1 |   1   |   2   |        2 |
        //    7 |   2 | 3 | 4 | 5 | 6 |        4 |
        // -----------|---|---|---|---|----------|
        // elem_index | 0 | 1 | 2 | 3 |
        //

        let mut level_idx: usize = 0;
        let mut elem_all_num: usize = 1;
        let mut elem_num: usize = 1;
        for i in 0..elem_total_num {
            // let elem_num = (elem_all_num << 1) - elem_all_num;
            // if elem_num == 0 {
            //     panic!("elem_num == 0: n = {}, i = {}", elem_all_num, i);
            // }
            let elem_index = i - (elem_all_num - elem_num);
            let elem_size = self.n / elem_num;

            let c: usize = elem_index * elem_size + elem_size / 2;
            let val = self.seg[i];
            display_vals[level_idx][c] = format!("{}", val.value());

            if i + 1 == elem_all_num {
                elem_num <<= 1;
                elem_all_num += elem_num;
                level_idx += 1;
            }
        }

        display_vals = padding_table(&display_vals);

        let mut result = std::fmt::Result::Ok(());
        for i in 0..tree_level {
            result = writeln!(f, "{}", display_vals[i].join(" "));
            if result.is_err() {
                return result;
            }
        }
        result
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct MinMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone,
{
    value: T,
}

impl<T> Value<T> for MinMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone + std::fmt::Display,
{
    fn value(&self) -> T {
        self.value
    }
}

impl<T> Monoid<T> for MinMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone + std::fmt::Display,
{
    fn identity() -> Self {
        Self {
            value: T::max_value(),
        }
    }

    // Binary operation
    fn op(&self, other: &Self) -> Self {
        Self {
            value: std::cmp::min(self.value(), other.value()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct MaxMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone,
{
    value: T,
}

impl<T> Value<T> for MaxMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone + std::fmt::Display,
{
    fn value(&self) -> T {
        self.value
    }
}

impl<T> Monoid<T> for MaxMonoid<T>
where
    T: num::Integer + num::Bounded + Copy + Clone + std::fmt::Display,
{
    fn identity() -> Self {
        Self {
            value: T::min_value(),
        }
    }

    // Binary operation
    fn op(&self, other: &Self) -> Self {
        Self {
            value: std::cmp::max(self.value(), other.value()),
        }
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

    type T = usize;
    let mut ans_ruq = SegmentTree::<T, MinMonoid<T>>::new(NUM_S);
    // kd: kindergarten
    let mut kids2kg = in_ab.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    let mut kg_each_ruq = vec![SegmentTree::<T, MaxMonoid<T>>::new(in_n); NUM_S];

    let mut kg_each_num = vec![0; NUM_S];

    // initialize
    for (i, (a, b)) in in_ab.iter().enumerate() {
        kg_each_ruq[*b].update(i, i + 1, MaxMonoid::<T> { value: *a });
        kg_each_num[*b] += 1;
        let size = kg_each_ruq[*b].n;
        let val = kg_each_ruq[*b].query(0, size).value();
        if val == MaxMonoid::<T>::identity().value() {
            ans_ruq.update(*b, *b + 1, MinMonoid::<T>::identity());
        } else {
            ans_ruq.update(*b, *b + 1, MinMonoid::<T> { value: val });
        }
    }

    for &(kids_idx, c_next_kg) in in_cd.iter() {
        let c_current_kg = kids2kg[kids_idx];

        // erase old data
        kg_each_ruq[c_current_kg].update(kids_idx, kids_idx + 1, MaxMonoid::<T>::identity());

        // update each rate
        kg_each_ruq[c_next_kg].update(
            kids_idx,
            kids_idx + 1,
            MaxMonoid::<T> {
                value: in_ab[kids_idx].0,
            },
        );

        kids2kg[kids_idx] = c_next_kg;

        // update kids number
        kg_each_num[c_current_kg] -= 1;
        kg_each_num[c_next_kg] += 1;

        // update ans_ruq
        let mut val = kg_each_ruq[c_current_kg].query(0, in_n).value();
        if val == 0 {
            val = MinMonoid::<T>::identity().value();
        }
        ans_ruq.update(
            c_current_kg,
            c_current_kg + 1,
            MinMonoid::<T> { value: val },
        );

        let val = kg_each_ruq[c_next_kg].query(0, in_n).value();
        ans_ruq.update(c_next_kg, c_next_kg + 1, MinMonoid::<T> { value: val });

        // println!("---------------------------------");
        // for i in 0..4 {
        //     println!("{}", &kg_each_ruq[i]);
        // }

        println!("{}", ans_ruq.query(0, NUM_S).value());
    }
}
