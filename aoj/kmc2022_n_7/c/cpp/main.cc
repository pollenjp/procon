#include <cstdint>
#include <cstdlib>
#include <cwchar>
#include <iostream>
#include <queue>
#include <vector>

using std::pair;
using std::priority_queue;
using std::vector;

const int kInf = 7 + (2e+9);
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

using Edges = vector<Edge>;
using Graph = vector<Edges>;

bool warshal_floyd(const Graph &g, vector<vector<Weight>> &dist, vector<vector<int>> &inter) {
  int n = g.size();
  dist.assign(n, vector<Weight>(n, kInf));
  for (int i = 0; i < n; i++) {
    dist[i][i] = 0;
  }
  for (int i = 0; i < n; i++) {
    for (auto f = g[i].begin(); f != g[i].end(); f++) {
      dist[i][f->dst] = f->weight;
    }
  }
  inter.assign(n, vector<int>(n, -1));
  for (int k = 0; k < n; k++) {
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        if (dist[i][k] < kInf && dist[k][j] < kInf && dist[i][j] > dist[i][k] + dist[k][j]) {
          dist[i][j] = dist[i][k] + dist[k][j];
          inter[i][j] = k;
        }
      }
    }
  }
  for (int i = 0; i < n; i++) {
    if (dist[i][i] < 0) {
      return false;
    }
  }
  return true;
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
  }

  std::vector<std::vector<Weight>> dist;
  std::vector<std::vector<int32_t>> prev;
  bool not_has_negative_cycle = warshal_floyd(graph, dist, prev);
  if (!not_has_negative_cycle) {
    std::cout << "NEGATIVE CYCLE" << std::endl;
    return EXIT_SUCCESS;
  }

  for (std::vector<Weight> &d_vec : dist) {
    // for (Weight d : d_vec) {
    for (uint32_t j{0}; j < d_vec.size(); j++) {
      auto d = d_vec[j];
      if (d >= kInf) {
        std::cout << "INF";
      } else {
        std::cout << d;
      }

      if (j < d_vec.size() - 1) {
        std::cout << " ";
      }
    }
    std::cout << std::endl;
  }
}
