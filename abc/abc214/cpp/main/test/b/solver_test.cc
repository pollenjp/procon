#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(1, 0), 4); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(2, 5), 10); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(10, 10), 213); }

TEST(SolverATest, Sample4) { EXPECT_EQ(solve(30, 100), 2471); }
