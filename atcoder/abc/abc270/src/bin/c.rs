use proconio::input;

use std::collections::BinaryHeap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge<T> {
    src: T,
    to: T,
    cost: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DijkstraEdge<T> {
    prev: T,
    current: T, // This may should be 'usize'
    cost: T,
}

impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for DijkstraEdge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl<T: std::cmp::Ord> std::cmp::Ord for DijkstraEdge<T> {
    fn cmp(&self, other: &DijkstraEdge<T>) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// i32::MAX

type Graph<T> = Vec<Vec<Edge<T>>>;

fn two<T>() -> T
where
    T: num::One + num::Integer,
{
    T::one() + T::one()
}

fn dijkstra<T>(graph: &Graph<T>, start: usize) -> (Vec<T>, Vec<T>)
where
    T: num::Integer
        + num::Signed
        + num::Bounded
        + num::Zero
        + num::FromPrimitive
        + num::ToPrimitive
        + std::cmp::PartialOrd
        + std::cmp::Ord
        + Copy
        + Clone,
{
    let mut dist = vec![T::max_value(); graph.len()];
    dist[start] = T::zero(); // 開始点は開始点からのコスト距離ゼロ

    // -1 : start
    // -2 : not connected
    let mut prevs = vec![-two::<T>(); graph.len()];
    prevs[start] = -T::one();

    let mut que = BinaryHeap::new();
    que.push(std::cmp::Reverse(DijkstraEdge::<T> {
        prev: -T::one(),
        current: T::from_usize(start).unwrap(),
        cost: T::zero(),
    }));

    while let Some(std::cmp::Reverse(e)) = que.pop() {
        let current_vertex = e.current.to_usize().unwrap();
        if dist[current_vertex] < e.cost {
            // visited
            continue;
        }

        for next_edge in graph[current_vertex].iter() {
            let next_vertex = next_edge.to.to_usize().unwrap();
            let expected_cost = dist[current_vertex] + next_edge.cost;
            if dist[next_vertex] > expected_cost {
                prevs[next_vertex] = e.current;
                dist[next_vertex] = expected_cost;
                que.push(std::cmp::Reverse(DijkstraEdge {
                    prev: T::from_usize(current_vertex).unwrap(),
                    current: T::from_usize(next_vertex).unwrap(),
                    cost: expected_cost,
                }));
            }
        }
    }
    (dist, prevs)
}

fn build_path<T>(prev: &Vec<T>, goal: usize) -> Vec<usize>
where
    T: num::Integer + num::Signed + num::One + num::FromPrimitive + num::ToPrimitive + Copy + Clone,
{
    if prev[goal] == -two::<T>() {
        // start から goal への道がない
        return vec![];
    }

    let mut path = vec![];
    let mut node = T::from_usize(goal).unwrap();

    // start に遡るまで loop
    while node != -T::one() {
        let v = node.to_usize().unwrap();
        path.push(v);
        node = prev[v];
    }
    path.reverse();
    path
}

fn main() {
    input! {
        in_n: usize,
        in_x: usize,
        in_y: usize,
        in_uv: [(usize, usize); in_n - 1],
    }

    type T = i32;
    let mut graph: Graph<T> = vec![vec![]; in_n];
    for &(u, v) in in_uv.iter() {
        graph[u - 1].push(Edge::<T> {
            src: u as T - 1,
            to: v as T - 1,
            cost: 1,
        });
        graph[v - 1].push(Edge::<T> {
            src: v as T - 1,
            to: u as T - 1,
            cost: 1,
        });
    }

    let (_, prev) = dijkstra(&graph, in_x - 1);
    let path = build_path(&prev, in_y - 1);
    for i in 0..path.len() {
        print!("{}", path[i] + 1);
        if i + 1 < path.len() {
            print!(" ");
        } else {
            println!("");
        }
    }
}
