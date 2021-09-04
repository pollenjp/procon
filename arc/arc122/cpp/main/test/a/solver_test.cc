#include "a/solver.h"

#include "gtest/gtest.h"

namespace arc122 {

TEST(SolverATest, Sample1) {
  int64_t n(3);
  std::vector<int64_t> vec({3, 1, 5});
  EXPECT_EQ(solve(n, vec), 15);
}

TEST(SolverATest, Sample2) {
  int64_t n(4);
  std::vector<int64_t> vec({1, 1, 1, 1});
  EXPECT_EQ(solve(n, vec), 10);
}

TEST(SolverATest, Sample3) {
  int64_t n(10);
  std::vector<int64_t> vec(
      {866111664, 178537096, 844917655, 218662351, 383133839, 231371336, 353498483, 865935868, 472381277, 579910117});
  EXPECT_EQ(solve(n, vec), 279919144);
}

}  // namespace arc122
