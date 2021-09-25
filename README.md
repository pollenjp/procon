# atcoder

## C++

using Bazel, glog (+ gflags)

```sh
bazelisk run //arc/arc127/cpp:main_a -- --log_dir $(pwd)/logs
bazelisk test --test_output=all //arc/arc127/cpp:test_a
```

## Python

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
