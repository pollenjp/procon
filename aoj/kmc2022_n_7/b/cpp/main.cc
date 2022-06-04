#include <cstdint>
#include <cwchar>
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

const int64_t kInf = 1e12;

bool comp(Edge l, Edge r) { return l.w > r.w; }

pair<vector<int64_t>, vector<int32_t>> dijkstra(const Graph& graph, int32_t start) {
  auto n = static_cast<int32_t>(graph.size());
  vector<int32_t> prevs(n, -1);   // 各頂点の直前に通る頂点を集める配列, 復元で使える
  vector<int64_t> dist(n, kInf);  // 解の配列。始点から各点までの最短距離を格納。
  dist[start] = 0;                // 始点から始点の距離は0

  // 始点の前に通る頂点として-2を入れておく
  // prevsの初期値で始点から到達されないことを意味する-1とは区別する
  // (始点を場合分けすれば求まるはずなのでやってもやらなくてもよいが、今回はvisitedのかわりにも使うので要)
  priority_queue pq(comp, vector<Edge>({{-2, start, 0}}));
  while (!pq.empty()) {
    auto [prev, cur, d] = pq.top();
    pq.pop();

    if (prevs[cur] != -1) continue;  // visitedのかわり

    prevs[cur] = prev;
    for (auto next_edge : graph[cur]) {
      int32_t to = next_edge.to;
      int32_t nx_weight = dist[cur] + next_edge.w;  // priority_queueに入れるのは"始点からの距離"
      if (dist[to] < nx_weight) continue;           // 確定済みの頂点はこれで十分キューに入らない
      dist[to] = nx_weight;
      pq.push({cur, to, nx_weight});
    }
  }
  return {dist, prevs};
}

int32_t main() {
  int32_t num_v(0);
  int32_t num_e(0);
  int32_t start(0);
  std::cin >> num_v >> num_e >> start;

  Graph graph = Graph(num_v);
  for (int32_t i = 0; i < num_e; i++) {
    int32_t s(0);
    int32_t t(0);
    int32_t w(0);
    std::cin >> s >> t >> w;
    graph[s].emplace_back(Edge{s, t, w});
  }

  auto ans = dijkstra(graph, start).first;
  for (auto& a : ans) {
    if (a == kInf) {
      std::cout << "INF" << std::endl;
    } else {
      std::cout << a << std::endl;
    }
  }
}
