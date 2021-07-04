
#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

namespace {

std::vector<long long> solve(const int N, const long long K,
                             const std::vector<int> vec) {
  long long left(K);
  long long n_everyone_have(0);
  int i = 0;
  if (K >= N) {
    n_everyone_have = K / N;
    left = K % N;
  }

  // init answer
  std::vector<long long> answer(N, n_everyone_have);

  // (index, id) pair's vector
  std::vector<std::pair<int, long long>> v(N, {0, 0});
  for (i = 0; i < N; i++) {
    v[i].first = i;
    v[i].second = static_cast<long long>(vec[i]);
  }
  // ID をもとに昇順に並べ替える
  // O(NlogN)
  std::sort(
      v.begin(), v.end(),
      [](const std::pair<int, long long> a, const std::pair<int, long long> b) {
        return a.second < b.second;
      });

  // update answer
  for (i = 0; i < left; i++) {
    auto answer_idx = v[i].first;
    answer[answer_idx]++;
  }
  return answer;
}
}  // namespace
