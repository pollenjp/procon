load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library", "cc_test")

#####
# A #
#####

cc_library(
    name = "SolverA",
    srcs = [],
    hdrs = [
        "src/a/solver.h",
    ],
    includes = ["src"],
    deps = [],
)

cc_binary(
    name = "main_a",
    srcs = glob(
        include = [
            "src/main_a.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverA",
    ],
)

cc_test(
    name = "test_a",
    timeout = "short",
    srcs = glob(
        include = [
            "test/a/solver_test.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverA",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)

#####
# B #
#####

cc_library(
    name = "SolverB",
    srcs = [],
    hdrs = [
        "src/b/solver.h",
    ],
    includes = ["src"],
    deps = [],
)

cc_binary(
    name = "main_b",
    srcs = glob(
        include = [
            "src/main_b.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverB",
    ],
)

cc_test(
    name = "test_b",
    timeout = "short",
    srcs = glob(
        include = [
            "test/b/solver_test.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverB",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)

#####
# C #
#####

cc_library(
    name = "SolverC",
    srcs = [],
    hdrs = [
        "src/c/solver.h",
    ],
    includes = ["src"],
    deps = [],
)

cc_binary(
    name = "main_c",
    srcs = glob(
        include = [
            "src/main_c.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverC",
    ],
)

#####
# D #
#####

cc_library(
    name = "SolverD",
    srcs = [],
    hdrs = [
        "src/d/solver.h",
    ],
    includes = ["src"],
    deps = [],
)

cc_binary(
    name = "main_d",
    srcs = glob(
        include = [
            "src/main_d.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverD",
    ],
)
