#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) { EXPECT_TRUE(solve(std::string("7777"))); }

TEST(SolverBTest, Sample2) { EXPECT_FALSE(solve(std::string("0112"))); }

TEST(SolverBTest, Sample3) { EXPECT_TRUE(solve(std::string("9012"))); }

TEST(SolverBTest, MySample1) { EXPECT_FALSE(solve(std::string("0189"))); }
