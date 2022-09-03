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
    current: T,
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

type Graph = Vec<Vec<Edge<i32>>>;

fn dijkstra(graph: &Graph, start: usize) -> (Vec<i32>, Vec<i32>) {
    let mut dist = vec![std::i32::MAX; graph.len()];
    dist[start] = 0; // 開始点は開始点からのコスト距離ゼロ
    let mut prevs = vec![-1; graph.len()];

    let mut que = BinaryHeap::new();
    que.push(std::cmp::Reverse(DijkstraEdge {
        prev: -2,
        current: start as i32,
        cost: 0,
    }));

    // let mut pq = BinaryHeap::<Edge>::new();

    while let Some(std::cmp::Reverse(e)) = que.pop() {
        // dbg!(&que);

        let (prev, cur) = (e.prev, e.current as usize);
        if prevs[cur] != -1 {
            // visited
            continue;
        }

        prevs[cur] = prev;
        for next_edge in &graph[cur] {
            let next = next_edge.to as usize;
            let next_cost = dist[cur] + next_edge.cost;
            if dist[next] > next_cost {
                dist[next] = next_cost;
                que.push(std::cmp::Reverse(DijkstraEdge {
                    prev: cur as i32,
                    current: next as i32,
                    cost: next_cost,
                }));
            }
        }
    }
    (dist, prevs)
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
    }

    let mut graph: Graph = vec![vec![]; in_n];

    for _ in 0..in_m {
        input! {
            src: usize,
            to: usize,
            cost: i32,
        }
        graph[src - 1].push(Edge {
            src: src as i32 - 1,
            to: to as i32 - 1,
            cost: cost,
        });
        graph[to - 1].push(Edge {
            src: to as i32 - 1,
            to: src as i32 - 1,
            cost: cost,
        });
    }

    // dbg!(&graph);

    let (dist_from_start, _) = dijkstra(&graph, 0);
    let (dist_from_goal, _) = dijkstra(&graph, in_n - 1);

    // dbg!(&dist_from_start);
    // dbg!(&dist_from_goal);

    for i in 0..in_n {
        // dbg!(dist_from_start[i], dist_from_goal[i]);
        println!("{}", dist_from_start[i] + dist_from_goal[i]);
    }
}
