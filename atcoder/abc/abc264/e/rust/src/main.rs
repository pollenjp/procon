use proconio::input;

// 頂点重み算出機能付き Union-Find
struct UnionFind<T> {
    // 親の頂点index
    parents: Vec<usize>,
    // 自身とその頂点が持つ子以下の頂点数の和
    sizes: Vec<usize>,
    // 自身とその頂点が持つ子以下の頂点重みの和
    weights: Vec<T>,
}

impl<T> UnionFind<T>
where
    T: Clone + Copy + Default + std::ops::AddAssign,
{
    fn new(n: usize, weights: Option<Vec<T>>) -> Self {
        let weights2: Vec<T>;
        match weights {
            None => {
                weights2 = vec![Default::default(); n];
            }
            Some(ws) => {
                weights2 = ws;
            }
        }
        UnionFind {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            weights: weights2,
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        self.parents[x] = self.root(self.parents[x]);
        self.parents[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.sizes[parent] < self.sizes[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.parents[child] = parent;
        self.sizes[parent] += self.sizes[child];
        let child_w = self.weights[child];
        self.weights[parent] += child_w;

        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.sizes[root]
    }

    fn weight(&mut self, x: usize) -> T {
        let root = self.root(x);
        self.weights[root]
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct UFWeight {
    cnt_city: usize,
    cnt_plant: usize,
}

impl std::ops::AddAssign for UFWeight {
    fn add_assign(&mut self, other: UFWeight) {
        self.cnt_city += other.cnt_city;
        self.cnt_plant += other.cnt_plant;
    }
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_e: usize,
        in_uv: [(usize, usize); in_e],
        in_q: usize,
        in_queries: [usize; in_q],
    }

    let num_vertices = in_n + in_m;

    let in_uv = in_uv
        .iter()
        .map(|&(u, v)| (u - 1, v - 1))
        .collect::<Vec<_>>();
    let in_queries = in_queries.iter().map(|&q| q - 1).collect::<Vec<_>>();

    let mut is_removed_edge = vec![false; in_e];
    for &query in in_queries.iter() {
        is_removed_edge[query] = true;
    }

    // val1: 連結している都市数
    // val2: 連結している発電所数
    let mut weights = vec![
        UFWeight {
            cnt_city: 1,
            cnt_plant: 0
        };
        in_n
    ];
    weights.append(&mut vec![
        UFWeight {
            cnt_city: 0,
            cnt_plant: 1
        };
        in_m
    ]);

    let mut uf = UnionFind::new(num_vertices, Some(weights));

    for (i, &(u, v)) in in_uv.iter().enumerate() {
        if is_removed_edge[i] {
            continue;
        }
        uf.unite(u, v);
    }

    let mut current_city_count = 0;
    for i in 0..num_vertices {
        if uf.root(i) != i {
            continue;
        }
        let w = uf.weight(i);
        if w.cnt_plant > 0 {
            current_city_count += w.cnt_city;
        }
    }

    let mut ans = Vec::with_capacity(in_q);
    for q in (0..in_q).rev() {
        ans.push(current_city_count);

        let query = in_queries[q];
        let (u, v) = in_uv[query];
        if uf.is_same(u, v) {
            continue;
        }

        let u_w_pre = uf.weight(u);
        let v_w_pre = uf.weight(v);

        if u_w_pre.cnt_plant > 0 {
            current_city_count -= u_w_pre.cnt_city;
        }
        if v_w_pre.cnt_plant > 0 {
            current_city_count -= v_w_pre.cnt_city;
        }

        uf.unite(u, v);

        let w = uf.weight(u);
        if w.cnt_plant > 0 {
            current_city_count += w.cnt_city;
        }
    }

    for a in ans.iter().rev() {
        println!("{}", a);
    }
}
