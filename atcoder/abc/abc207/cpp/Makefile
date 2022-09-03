DATA_DIR := "../data"
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress

export

check-all :
	${MAKE} check-c

.PHONY:check-c
check-c:
	bazel run ${BAZEL_RUN_OPTIONS} //main:main_c < ${DATA_DIR}/c/in1.txt


.PHONY:test
test:
	bazel test --test_output=all $(shell bazel query //...)
