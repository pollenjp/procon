#include "a/solver.h"

#include "gtest/gtest.h"

#define EPSILON 1e-5

TEST(SolverATest, Sample1) { EXPECT_EQ(solve(50, 50), std::string("Alloy")); }

TEST(SolverATest, Sample2) { EXPECT_EQ(solve(100, 0), std::string("Gold")); }

TEST(SolverATest, Sample3) { EXPECT_EQ(solve(0, 100), std::string("Silver")); }

TEST(SolverATest, Sample4) { EXPECT_EQ(solve(100, 2), std::string("Alloy")); }
