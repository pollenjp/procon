SHELL := $(shell which bash)

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
	${BAZEL_CMD} test //...

.PHONY: clean
clean:
	${BAZEL_CMD} clean

.PHONY: go-format
go-format:
	@find . -type f -regextype posix-extended \
		-regex ".*\.go" \
		-exec goimports -w -local ${GO_PACKAGE} {} \;

.PHONY: go-lint
go-lint:
	@find ${ROOT} -type d -regextype posix-extended \
		-regex ".*/section[0-9]{2}/step[0-9]{2}" \
		-print0 | \
		while IFS= read -r -d '' line; do \
			old_cwd=$(pwd) ; \
			cd $$line ; \
			echo "linting $$line" ; \
			staticcheck ./... ; \
			go vet ./... ; \
			cd $$old_cwd ; \
		done
