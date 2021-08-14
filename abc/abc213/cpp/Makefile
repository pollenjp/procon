DATA_DIR := ../data
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress
TASK_NAME_SYMBOL := a

export

check-all :
	${MAKE} check TASK_NAME=c
	${MAKE} check TASK_NAME=d

.PHONY:check
check:
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_${TASK_NAME_SYMBOL} < ${DATA_DIR}/${TASK_NAME_SYMBOL}/in1.txt
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_${TASK_NAME_SYMBOL} < ${DATA_DIR}/${TASK_NAME_SYMBOL}/in2.txt


.PHONY:test
test:
	bazel test --test_output=all $(shell bazel query //...)
