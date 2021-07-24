#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) {
  std::vector<std::string> s_vec({"3B", "HR", "2B", "H"});
  EXPECT_TRUE(solve(s_vec));
}
TEST(SolverBTest, Sample2) {
  std::vector<std::string> s_vec({"2B", "3B", "HR", "3B"});
  EXPECT_FALSE(solve(s_vec));
}
