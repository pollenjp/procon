use proconio::input;

#[derive(Debug, Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

type Graph = Vec<Vec<Edge>>;

fn main() {
    input! {
        in_n: usize,
        in_x: usize,
        in_y: usize,
        in_uv: [(usize, usize); in_n - 1],
    }

    let mut graph: Graph = vec![vec![]; in_n];
    for &(u, v) in in_uv.iter() {
        graph[u - 1].push(Edge {
            from: u - 1,
            to: v - 1,
            cost: 1,
        });
        graph[v - 1].push(Edge {
            from: v - 1,
            to: u - 1,
            cost: 1,
        });
    }

    let (_, prev) = dijkstra(&graph, in_x - 1);
    let path = build_path(&prev, in_y - 1);
    for i in 0..path.len() {
        print!("{}", path[i] + 1);
        if i + 1 < path.len() {
            print!(" ");
        }
    }
}

fn dijkstra(graph: &Graph, start: usize) -> (Vec<i64>, Vec<i64>) {
    let mut prev: Vec<i64> = vec![-2; graph.len()]; // 直前に通る頂点
    let mut dist: Vec<i64> = vec![std::i64::MAX; graph.len()];

    let mut heap = std::collections::BinaryHeap::new();
    heap.push(std::cmp::Reverse((0, start)));

    dist[start] = 0;
    while let Some(std::cmp::Reverse((cost, node))) = heap.pop() {
        if dist[node] < cost {
            continue;
        }
        for edge in &graph[node] {
            let next_cost = cost + edge.cost;
            if next_cost < dist[edge.to] {
                prev[edge.to] = node as i64;
                dist[edge.to] = next_cost;
                heap.push(std::cmp::Reverse((next_cost, edge.to)));
            }
        }
    }
    (dist, prev)
}

fn build_path(prev: &Vec<i64>, goal: usize) -> Vec<usize> {
    if prev[goal] == -1 {
        // start から goal への道がない
        return vec![];
    }

    let mut path = vec![];
    let mut node = goal as i64;
    while node != -2 {
        path.push(node as usize);
        node = prev[node as usize];
    }
    path.reverse();
    path
}
