DATA_DIR := "./data"
BAZEL_RUN_OPTIONS := \
	--ui_event_filters=-info,-stdout,-stderr \
	--noshow_progress

.PHONY:test
test:
	bazel test --test_output=all $(shell bazel query //...)
