#include "pollenlib/solver.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace typical90 {

namespace b {

TEST(SolverCTest, Sample1) { EXPECT_EQ(solve(2).size(), 1); }

TEST(SolverCTest, Sample2) { EXPECT_EQ(solve(3).size(), 0); }

TEST(SolverCTest, Sample3) { EXPECT_EQ(solve(10).size(), 42); }

}  // namespace b

}  // namespace typical90

}  // namespace atcoder
