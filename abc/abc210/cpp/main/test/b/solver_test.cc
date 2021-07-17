#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_EQ(solve(5, "00101"), "Takahashi"); }
TEST(SolverBTest, Sample2) { EXPECT_EQ(solve(3, "010"), "Aoki"); }
