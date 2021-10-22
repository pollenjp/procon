#include "pollenlib/solver.h"

#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace edpc {

namespace c {

TEST(SolverCTest, Sample1) {
  int32_t num_n(3);
  std::vector<int32_t> a_vec({10, 20, 30});
  std::vector<int32_t> b_vec({40, 50, 60});
  std::vector<int32_t> c_vec({70, 80, 90});

  EXPECT_EQ(solve(num_n, a_vec, b_vec, c_vec), 210);
}

}  // namespace c

}  // namespace edpc

}  // namespace atcoder
