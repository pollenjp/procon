#include "pollenlib/solver.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace abc227 {

namespace c {

TEST(SolverCTest, Sample1) { EXPECT_EQ(solve(4LL), 5LL); }

TEST(SolverCTest, Sample2) { EXPECT_EQ(solve(100LL), 323LL); }

TEST(SolverCTest, Sample3) { EXPECT_EQ(solve(100000000000LL), 5745290566750LL); }

}  // namespace c

}  // namespace abc227

}  // namespace atcoder
