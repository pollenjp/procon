SHELL := bash

ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
export BAZEL_CMD := bazelisk
GO_PACKAGE := github.com/pollenjp/gohandson-gacha

.PHONY: build
build:
	${MAKE} gazelle
	${BAZEL_CMD} build //...

.PHONY: gazelle
gazelle:  ## gazelle によって設定ファイルを自動生成する
	${BAZEL_CMD} run //:gazelle-update-repos
	${BAZEL_CMD} run //:gazelle

.PHONY: test
test:
	${BAZEL_CMD} test --test_output=all //...

.PHONY: clean
clean:
	${BAZEL_CMD} clean

.PHONY: go-format
go-format:
	find . -type f -regextype posix-extended \
		-regex ".*\.go" \
		-exec goimports -w -local ${GO_PACKAGE} {} \;


.PHONY: python-lint
python-lint:
	isort --check .
	flake8 .
	black --check .

.PHONY: go-lint
go-lint:
	-staticcheck ./...
	-go vet ./...
