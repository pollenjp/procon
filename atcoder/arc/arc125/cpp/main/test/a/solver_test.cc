#include "a/solver.h"

#include "gtest/gtest.h"

namespace arc215 {

namespace {}  // namespace

TEST(SolverATest, Sample1) {
  std::vector<int> s_vec({0, 0, 1});
  std::vector<int> t_vec({0, 1, 1, 0});
  EXPECT_EQ(solve(s_vec, t_vec), 6);
}

TEST(SolverATest, Sample2) {
  std::vector<int> s_vec({0});
  std::vector<int> t_vec({1});
  EXPECT_EQ(solve(s_vec, t_vec), -1);
}

}  // namespace arc215
