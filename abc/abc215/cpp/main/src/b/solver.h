#include <algorithm>
#include <vector>

#define LOG_BASE 2

namespace {

long long solve(long long n) {
  long long k = 0;

  n /= LOG_BASE;
  while(n != 0){
    k++;
    n /= LOG_BASE;
  }

  return k;
}
}  // namespace
