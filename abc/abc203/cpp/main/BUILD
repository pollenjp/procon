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

########
# test #
########

cc_test(
    name = "solver_test",
    timeout = "short",
    srcs = glob(
        include = [
            "test/*/solver_test.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverA",
        ":SolverB",
        ":SolverC",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)
