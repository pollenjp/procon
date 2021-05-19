DATA_DIR := "./data"
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress

check-all:
	${MAKE} check-b

.PHONY:check-b
check-b:
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_b < ${DATA_DIR}/b/in1.txt
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_b < ${DATA_DIR}/b/in2.txt
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_b < ${DATA_DIR}/b/in3.txt

.PHONY:test
test:
	bazel test --test_output=all $(shell bazel query //...)
