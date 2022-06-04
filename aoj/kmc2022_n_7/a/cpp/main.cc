#include <cstdint>
#include <iostream>
#include <queue>
#include <vector>

using std::pair;
using std::priority_queue;
using std::vector;

struct Edge {
  int32_t src;  // 有向辺の根本
  int32_t to;   // 有向辺の行き先
  int64_t w;    // 同辺の重み
};

using Graph = vector<vector<Edge>>;

// priority_queueに渡す比較関数
// これが一番直感に反すると思うんだけど、
// デフォルトだとpriority_queueは"<"の比較を見てmax heapを作る
// ので、逆の比較を渡すとmin heapになる。
bool comp(Edge l, Edge r) { return l.w > r.w; }
// 復元もやってる
pair<vector<Edge>, int64_t> prim(const Graph& graph, int32_t start) {
  auto n = static_cast<int32_t>(graph.size());
  vector<Edge> ans_edges;  // 解に使う辺を集める配列, 復元で使える
  int64_t ans_sum = 0;     // 解のコストの和
  vector<bool> visited(n);

  priority_queue pq(comp, vector<Edge>({{-1, start, 0}}));
  while (!pq.empty()) {
    auto cur_edge = pq.top();
    auto [prev, cur, w] = cur_edge;
    pq.pop();
    if (visited[cur]) continue;
    ans_edges.push_back(cur_edge);
    ans_sum += w;
    visited[cur] = true;
    for (auto next_Edge : graph[cur]) {
      int32_t to = next_Edge.to;
      if (visited[to]) continue;
      pq.push(next_Edge);
    }
  }
  return {ans_edges, ans_sum};
}

int32_t main() {
  int32_t num_v(0);
  int32_t num_e(0);
  std::cin >> num_v >> num_e;

  Graph graph = Graph(num_v);
  for (int32_t i = 0; i < num_e; i++) {
    int32_t s(0);
    int32_t t(0);
    int32_t w(0);
    std::cin >> s >> t >> w;
    graph[s].emplace_back(Edge{s, t, w});
    graph[t].emplace_back(Edge{t, s, w});
  }

  // for (auto& v_e : graph) {
  //   for (auto& e : v_e) {
  //     std::cout << e.src << " " << e.to << " " << e.w << std::endl;
  //   }
  // }
  std::cout << prim(graph, 0).second << std::endl;
}
