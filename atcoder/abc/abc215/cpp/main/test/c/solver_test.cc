#include "c/solver.h"

#include "gtest/gtest.h"


TEST(SolverCTest, Sample1) {EXPECT_EQ(solve("aab" ,2), "aba");}

TEST(SolverCTest, Sample2) {EXPECT_EQ(solve("baba" ,4), "baab");}

TEST(SolverCTest, Sample3) {EXPECT_EQ(solve("ydxwacbz" ,40320), "zyxwdcba");}

 
