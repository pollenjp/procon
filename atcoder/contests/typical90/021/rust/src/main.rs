use proconio::input;
use std::collections::HashSet;

type Graph = Vec<Vec<usize>>;

struct StronglyConnectedComponents {
    // graph
    // ng[i] は i から行ける先の頂点のインデックスのvecを格納する
    // g: Graph,
    ng: Graph,            // normal graph
    rg: Graph,            // reverse graph
    components: Vec<i64>, // 頂点が属するコンポーネント. -1 は未所属
    used: Vec<bool>,
    order: Vec<usize>, // 頂点のラベリング順序
}

impl StronglyConnectedComponents {
    fn new(g: Graph) -> Self {
        let ng: Graph = g.clone();
        let mut rg: Graph = vec![vec![]; g.len()];

        for i in 0..g.len() {
            for j in g[i].clone() {
                rg[j].push(i);
            }
        }
        Self {
            ng,
            rg,
            components: vec![-1; g.len()],
            used: vec![false; g.len()],
            order: Vec::with_capacity(g.len()),
        }
    }

    fn get_component_num(self: &Self, i: usize) -> i64 {
        self.components[i]
    }

    fn dfs(&mut self, i: usize) {
        if self.used[i] {
            return;
        }
        self.used[i] = true;
        for j in self.ng[i].clone() {
            self.dfs(j);
        }
        self.order.push(i);
    }

    // reverse dfs
    fn rdfs(&mut self, i: usize, count: i64) {
        if self.components[i] != -1 {
            return;
        }
        self.components[i] = count;
        for j in self.rg[i].clone() {
            self.rdfs(j, count);
        }
    }

    fn scc(&mut self) {
        for i in 0..self.ng.len() {
            self.dfs(i);
        }

        // dbg!(&self.order);

        self.order.reverse();

        let mut group = 0;
        for i in self.order.clone() {
            self.rdfs(i, group);
            // dbg!(&i);
            group += 1;
        }
    }

    fn is_same_component(&self, a: usize, b: usize) -> bool {
        self.components[a] == self.components[b]
    }
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_ab: [(usize, usize); in_m],
    }

    let in_ab = in_ab
        .into_iter()
        .map(|(a, b)| ((a as i64 - 1) as usize, (b as i64 - 1) as usize))
        .collect::<Vec<_>>();

    let mut g = vec![HashSet::<usize>::new(); in_n];
    for (a, b) in in_ab {
        g[a].insert(b);
    }

    let g: Graph = g
        .into_iter()
        .map(|s| s.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // dbg!(&g);

    let mut scc = StronglyConnectedComponents::new(g);
    scc.scc();

    let mut group_vertices =
        vec![vec![]; scc.components.iter().map(|&x| x as i64).max().unwrap() as usize + 1];
    for it in scc.components.iter().enumerate() {
        let (i, &component) = it;
        group_vertices[component as usize].push(i);
    }
    // dbg!(&scc.components);

    let mut cnt = 0;
    for vertices in group_vertices {
        // dbg!(&vertices);
        if vertices.len() < 1 {
            continue;
        }
        cnt += vertices.len() * (vertices.len() - 1) / 2;
    }
    println!("{}", cnt);
}
