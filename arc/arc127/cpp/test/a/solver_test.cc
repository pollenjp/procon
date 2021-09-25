#include "a/solver.h"

#include "gtest/gtest.h"

namespace arc127 {

TEST(SolverATest, Sample1) {
  int64_t input_n(11);
  int64_t ans(4);
  EXPECT_EQ(solve(input_n), ans);
}

TEST(SolverATest, Sample2) {
  int64_t input_n(120);
  int64_t ans(44);
  EXPECT_EQ(solve(input_n), ans);
}

TEST(SolverATest, Sample3) {
  int64_t input_n(987654321);
  int64_t ans(123456789);
  EXPECT_EQ(solve(input_n), ans);
}

TEST(SolverATest, AtCoder_01_005) {
  int64_t input_n(363225189037284);
  int64_t ans(123456790123455);
  EXPECT_EQ(solve(input_n), ans);
}

TEST(SolverATest, AtCoder_01_006) {
  int64_t input_n(642087521202247);
  int64_t ans(123456790123455);
  EXPECT_EQ(solve(input_n), ans);
}

}  // namespace arc127
