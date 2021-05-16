DATA_DIR := "./data"
BAZEL_RUN_OPTIONS := --ui_event_filters=-info,-stdout,-stderr --noshow_progress

check-all:
	${MAKE} check-a

.PHONY:check-a
check-a:
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/ex1_in.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/ex2_in.txt
