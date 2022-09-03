DATA_DIR := "./data"
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress

check-all:
	${MAKE} check-a

.PHONY:check-a
check-a:
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/in1.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/in2.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/in3.txt
	bazel run ${BAZEL_RUN_OPTIONS} //src:main_a < ${DATA_DIR}/a/in4.txt
