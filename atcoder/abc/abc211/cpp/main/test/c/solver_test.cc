#include "c/solver.h"

#include <vector>

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) { EXPECT_EQ(solve(std::string("chchokudai")), 3); }
TEST(SolverCTest, Sample2) { EXPECT_EQ(solve(std::string("atcoderrr")), 0); }
TEST(SolverCTest, Sample3) {
  EXPECT_EQ(solve(std::string("chokudaichokudaichokudai")), 45);
}
