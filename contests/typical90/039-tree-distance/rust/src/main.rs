use proconio::input;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct TreeDP<T>
where
    T: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    // dp[i] : root node を除いた node 'i' に向けた edge の寄与率 (貢献度). n-1本.
    //         当然 root node の index の edge は存在しないので寄与率 0.
    dp: Vec<T>,
    graph: Vec<Vec<usize>>,
}

impl<T> TreeDP<T>
where
    T: num::Integer + num::Bounded + num::Zero + num::One + Copy + Clone,
{
    fn get_no_parent_index() -> T {
        return T::max_value();
    }

    fn new(n: usize) -> Self {
        TreeDP {
            dp: vec![T::zero(); n],
            graph: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
    }

    fn dfs(&mut self, v: usize, parent: usize) -> T {
        let mut val = T::zero();
        for i in 0..self.graph[v].len() {
            let to = self.graph[v][i];
            if to == parent {
                continue;
            }
            self.dp[to] = self.dfs(to, v);
            val = val + self.dp[to];
        }

        val + T::one()
    }
}

fn main() {
    input! {
        in_n: usize,
        in_ab: [(usize, usize); in_n-1],
    }

    let mut tree = TreeDP::<usize>::new(in_n);

    for (a, b) in in_ab {
        let a = a - 1;
        let b = b - 1;
        tree.add_edge(a, b);
        tree.add_edge(b, a);
    }

    tree.dfs(0, TreeDP::<usize>::get_no_parent_index());

    let mut ans = 0;
    for val in tree.dp {
        ans += val * (in_n - val);
    }
    println!("{}", ans);
}
