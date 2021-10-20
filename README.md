# atcoder

<https://scrapbox.io/pollenJP-Memo/AtCoder>

## Execution

### C++

using Bazel, glog (+ gflags)

run code

```sh
logdir=$(realpath logs_glog);\
  mkdir -p ${logdir} && \
  bazelisk run contests/dp/cpp:main_a -- --log_dir ${logdir}
```

run test

```sh
bazelisk test --test_output=all contests/dp/cpp:test_a:test_a
```

### go

using gazelle

```sh
make build
```

```sh
bazelisk run contests/practice/golang/a
```

or directly `go run`

```sh
cd contests/practice/golang/a
go run .
```

### Python

- Python version : 3.8.2
  - example

    ```sh
    pyenv install 3.8.2
    pyenv virtualenv 3.8.2 py3.8.2-atcoder
    pyenv local py3.8.2-atcoder
    ```

- pipenv

  ```sh
  pip install -U pip
  pip install -U pipenv
  ```

- requirements

  ```sh
  pipenv install --dev
  ```
