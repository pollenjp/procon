#pragma once

#include <algorithm>
#include <vector>

namespace {

long long solve(int n, long long k,
                std::vector<std::pair<long long, long long>> &ab_vectors) {
  std::sort(ab_vectors.begin(), ab_vectors.end());

  long long stock(k);
  long long current_place(0);
  for (auto itr = ab_vectors.begin(); itr != ab_vectors.end(); itr++) {
    long long tmp = stock - ((*itr).first - current_place);
    if (tmp >= 0) {
      stock = tmp + (*itr).second;
      current_place = (*itr).first;
    } else {
      break;
    }
  }
  // overflow しないか？ -> しない (ref: 公式解説)
  return stock + current_place;
}

}  // namespace
