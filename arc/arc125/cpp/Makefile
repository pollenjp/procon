CMAKE_CMD := cmake
CMAKE_SOURCE_PATH := main/
CMAKE_BUILD_PATH := build
CMAKE_GENERATOR := "Unix Makefiles"

CMAKE_CXX_FLAGS := \
	-std=c++17 \
	-stdlib=libc++ \
	-Weverything \
	-Wno-c++98-compat
CMAKE_OPTIONS := \
    -D CMAKE_C_COMPILER=${HOME}/.local/bin/clang \
    -D CMAKE_CXX_COMPILER=${HOME}/.local/bin/clang++ \
    -D CMAKE_CXX_FLAGS="${CMAKE_CXX_FLAGS}"
CLANG_TIDY_CMD := clang-tidy
RUN_CLANG_TIDY_CMD := run-clang-tidy

export

.DEFAULT_GOAL: all

.PHONY: all
all:
	${MAKE} clean
	${MAKE} cmake
	${MAKE} cmake-export-compile-commands
	${MAKE} cmake-build


.PHONY: cmake
cmake:
	${CMAKE_CMD} \
		${CMAKE_OPTIONS} \
		-G ${CMAKE_GENERATOR} \
		-S ${CMAKE_SOURCE_PATH} \
		-B ${CMAKE_BUILD_PATH}

.PHONY: cmake-export-compile-commands
cmake-export-compile-commands:
	${MAKE} cmake \
		CMAKE_OPTIONS='${CMAKE_OPTIONS} -D CMAKE_EXPORT_COMPILE_COMMANDS=ON'

.PHONY: cmake-build
cmake-build:
	${CMAKE_CMD} --build ${CMAKE_BUILD_PATH}

.PHONY: clang-tidy
clang-tidy:
	${CLANG_TIDY_CMD} -p ${CMAKE_BUILD_PATH} ./student/src/main.cpp

.PHONY: run-clang-tidy
run-clang-tidy:
	${RUN_CLANG_TIDY_CMD} \
		-clang-tidy-binary ${CLANG_TIDY_CMD} \
		-p ${CMAKE_BUILD_PATH} \
		-j 4

.PHONY: clean
clean:
	-rm -rf ${CMAKE_BUILD_PATH}
