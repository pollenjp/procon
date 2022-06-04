#include <cstdint>
#include <cstdlib>
#include <cwchar>
#include <iostream>
#include <queue>
#include <vector>

using std::pair;
using std::priority_queue;
using std::vector;

const int kInf = 7 + (1e+9);
using Weight = int32_t;

struct Edge {  // src:辺の始点,dst:辺の終点,weight:辺の重さ
  int32_t src;
  int32_t dst;
  Weight weight;
  Edge(int32_t src, int32_t dst, Weight weight) : src(src), dst(dst), weight(weight) {}
};

bool operator<(const Edge &e, const Edge &f) {
  return e.weight != f.weight ? e.weight > f.weight :  //辺は重さが重いものを"小さい"と定義する
             e.src != f.src ? e.src < f.src
                            : e.dst < f.dst;
}

struct UnionFind {   // rankで高速化
  vector<int> par;   // parは添字の親（または自分が根の時自身）を示す。
  vector<int> rank;  // rankは木の高さ（根についてのみ実装）
  UnionFind(int sz) {
    par.resize(sz);
    rank.assign(sz, 0);
    for (int i = 0; i < sz; ++i) par[i] = i;
  }
  int find(int a) {  //根を探す
    if (par[a] == a) {
      return a;
    }
    return find(par[a]);
  }
  void unite(int a, int b) {  //小さい方の木の根に大きい方の根を上書きする
    int root_a = find(a), root_b = find(b);
    if (root_a == root_b) return;  //既に同じなら終了
    if (rank[root_a] > rank[root_b]) {
      par[root_b] = root_a;
    } else if (rank[root_a] < rank[root_b]) {
      par[root_a] = root_b;
    } else {
      rank[root_a]++;
      par[root_b] = root_a;
    }
  }
};

using Edges = vector<Edge>;
using Graph = vector<Edges>;

bool bellman_ford(const Graph &g, int s, vector<Weight> &dist, vector<int> &prev, vector<bool> &is_connected) {
  auto n = static_cast<int32_t>(g.size());
  dist.assign(n, kInf + kInf);
  dist[s] = 0;
  is_connected.assign(n, false);
  is_connected[s] = true;
  prev.assign(n, -2);
  bool negative_cycle = false;
  for (int k = 0; k < n; k++) {
    for (int i = 0; i < n; i++) {
      for (auto e = g[i].begin(); e != g[i].end(); e++) {
        if (is_connected[e->src]) {
          is_connected[e->dst] = true;
        }
      }
    }
  }
  for (int k = 0; k < n; k++) {
    for (int i = 0; i < n; i++) {
      for (auto e = g[i].begin(); e != g[i].end(); e++) {
        if (is_connected[e->src] != is_connected[e->dst]) {
          continue;
        }
        if (dist[e->dst] > dist[e->src] + e->weight) {
          dist[e->dst] = dist[e->src] + e->weight;
          prev[e->dst] = e->src;
          if (k == n - 1) {
            dist[e->dst] = -kInf;
            negative_cycle = true;
          }
        }
      }
    }
  }
  return !negative_cycle;
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

  vector<Weight> dist;
  vector<int32_t> prev;
  vector<bool> is_connected;
  bellman_ford(graph, start, dist, prev, is_connected);

  // for (int32_t i(0); i < dist.size(); i++) {
  //   auto d = dist[i];
  //   std::cout << is_connected[i] << ":" << d << std::endl;
  // }
  for (int32_t i(0); i < dist.size(); i++) {
    auto d = dist[i];
    if (is_connected[i] && d == -kInf) {
      std::cout << "NEGATIVE CYCLE" << std::endl;
      return EXIT_SUCCESS;
    }
  }

  for (int32_t i(0); i < dist.size(); i++) {
    auto d = dist[i];
    auto p = prev[i];
    // std::cout << p << " ";
    if (is_connected[i]) {
      std::cout << d << std::endl;
    } else {
      std::cout << "INF" << std::endl;
    }
  }

  // std::cout << p << " ";
  // if (d >= kInf) {
  //   std::cout << "INF" << std::endl;
  // } else {
  //   std::cout << d << std::endl;
  // }
}
