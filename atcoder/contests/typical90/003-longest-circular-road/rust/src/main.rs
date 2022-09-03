extern crate num;

use proconio::input;

// Monoid
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Value<T>
where
    T: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    value: T,
}

impl<T> std::ops::Add for Value<T>
where
    T: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    type Output = Value<T>;

    fn add(self, other: Self) -> Self {
        Value {
            value: self.value + other.value,
        }
    }
}

impl<T> Value<T>
where
    T: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    fn new(value: T) -> Self {
        Self { value }
    }

    // 単位元
    fn identity() -> Self {
        Self::new(T::min_value())
    }

    // 二項演算: Binary operation
    fn op(&self, other: &Self) -> Self {
        Self {
            value: std::cmp::max(self.value, other.value),
        }
    }
}

#[derive(Debug)]
struct ReRooting<ValueT>
where
    ValueT: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    // dp[i][j]: i を root としたときの j との関係値
    dp: Vec<Vec<Value<ValueT>>>,
    // diameters[i]: i を root としたときの木の直径
    diameters: Vec<Value<ValueT>>,
    // tree
    // graph[i]p[j]: node i の隣接ノードのうち j 番目への有向辺
    graph: Vec<Vec<Edge<ValueT>>>,
}

#[derive(Clone, Debug)]
struct Edge<ValueT>
where
    ValueT: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    to: usize,
    weight: Value<ValueT>,
}

impl<ValueT> ReRooting<ValueT>
where
    ValueT: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    fn new(n: usize) -> Self {
        let dp = vec![vec![]; n];
        let diameters = vec![Value::<ValueT>::identity(); n];
        // let graph:  = vec![vec![]; n];
        let graph: Vec<Vec<Edge<ValueT>>> = vec![vec![]; n];
        ReRooting {
            dp,
            diameters,
            graph,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: Value<ValueT>) {
        self.graph[from].push(Edge { to, weight });
    }

    fn build(&mut self) {
        self.dfs(0, usize::max_value(), &mut Value::<ValueT>::identity());
        self.bfs(0, Value::<ValueT>::identity(), usize::max_value());
    }

    // tree dp
    // parent: usize. std::usize::MAX if root (root has no parents)
    fn dfs(&mut self, v: usize, parent: usize, weight: &Value<ValueT>) -> Value<ValueT> {
        let mut val = Value::<ValueT>::identity();
        let num_of_adj_nodes = self.graph[v].len();
        self.dp[v] = vec![Value::<ValueT>::identity(); num_of_adj_nodes];
        for i in 0..num_of_adj_nodes {
            let Edge {
                to,
                weight: to_weight,
            } = self.graph[v][i];
            if to == usize::max_value() {
                panic!("to should not be std::usize::MAX");
            }
            if to == parent {
                continue;
            }
            self.dp[v][i] = self.dfs(to, v, &to_weight);
            val = val.op(&self.dp[v][i]);
        }

        val + *weight
    }

    // rerooting
    // v: node
    fn bfs(&mut self, v: usize, dp_p: Value<ValueT>, parent: usize) {
        // adjacent node
        for i in 0..self.graph[v].len() {
            let Edge { to, weight: _ } = self.graph[v][i];
            if to == std::usize::MAX {
                panic!("to should not be std::usize::MAX");
            }
            if to == parent {
                self.dp[v][i] = dp_p;
            }
        }

        let deg = self.graph[v].len();
        let mut dp_l = vec![Value::<ValueT>::identity(); deg + 1];
        let mut dp_r = vec![Value::<ValueT>::identity(); deg + 1];

        for i in 0..deg {
            dp_l[i + 1] = dp_l[i].op(&self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = dp_r[i + 1].op(&self.dp[v][i]);
        }

        self.diameters[v] = dp_l[deg];

        for i in 0..deg {
            let Edge {
                to,
                weight: to_weight,
            } = self.graph[v][i];
            if to == parent {
                continue;
            }
            self.bfs(to, dp_l[i].op(&dp_r[i + 1]) + to_weight, v);
        }
    }
}

fn main() {
    input! {
        in_n: usize,
        in_ab: [(usize, usize); in_n - 1],
    }

    let mut reroot = ReRooting::<usize>::new(in_n);

    for (a, b) in in_ab {
        let a = a - 1;
        let b = b - 1;
        reroot.add_edge(a, b, Value::<usize>::new(1));
        reroot.add_edge(b, a, Value::<usize>::new(1));
    }
    reroot.build();

    // for d in reroot.diameters {
    //     println!("{}", d.value + 1);
    // }
    let m = reroot.diameters.iter().max().unwrap().value;
    println!("{}", m + 1);
}
