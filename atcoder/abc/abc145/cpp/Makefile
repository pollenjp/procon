DATA_DIR := "./data"
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress

check-all:
	${MAKE} check-a
	${MAKE} check-b
	${MAKE} check-c

.PHONY:check-a
check-a:
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/ex1_in.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/ex2_in.txt

.PHONY:check-b
check-b:
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_b < ${DATA_DIR}/b/in1.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_b < ${DATA_DIR}/b/in2.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_b < ${DATA_DIR}/b/in3.txt

.PHONY:check-c
check-c:
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_c < ${DATA_DIR}/c/in1.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_c < ${DATA_DIR}/c/in2.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_c < ${DATA_DIR}/c/in3.txt
