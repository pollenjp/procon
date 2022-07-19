use proconio::input;

// state:
// 0: group1
// 1: group2
// -1: not searched yet
fn dfs(current_node: usize, parent_node: usize, graph: &Vec<Vec<usize>>, state: &mut Vec<i32>) {
    let current_state = state[current_node];
    if current_state == -1 {
        // error
    }

    let next_state = (1 - current_state).abs() as i32;

    for child_node in graph[current_node].clone() {
        if child_node == parent_node {
            continue;
        }
        state[child_node] = next_state;
        dfs(child_node, current_node, graph, state);
    }
}
fn main() {
    input! {
        in_n: usize,
        edges: [(usize, usize); in_n - 1],
    }
    let edges = edges
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<Vec<_>>();

    let mut graph = vec![vec![]; in_n];

    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut state = vec![-1; in_n];
    state[0] = 0; // root は not selected
    dfs(0, 0, &graph, &mut state);

    let mut label = 1;
    let filtered: Vec<&i32> = state.iter().filter(|&&s| s == label).collect::<Vec<_>>();
    if filtered.len() < in_n / 2 {
        label = 0;
    }

    let mut cnt = 0;
    for (i, &s) in state.iter().enumerate() {
        if s == label {
            print!("{} ", i + 1);
            cnt += 1;
            if cnt == in_n / 2 {
                break;
            }
        }
    }
}
