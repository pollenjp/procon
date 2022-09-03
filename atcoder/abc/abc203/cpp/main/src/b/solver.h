#pragma once

namespace {

int solve(const int n, const int k) {
  int counter(0);

  // pattern1
  //
  // for (int i = 0; i < n; i++) {
  //   for (int j = 0; j < k; j++) {
  //     counter += i * 100 + j;
  //   }
  // }
  // return counter;

  // pattern2
  //
  // 55 = 1 + 2 + ... + 9
  int tmp(0);
  for (int i = 1; i < n + 1; i++) {
    tmp += i;
  }
  counter += tmp * 100 * k;  // 百の位
  tmp = 0;
  for (int j = 1; j < k + 1; j++) {
    tmp += j;
  }
  counter += tmp * n;

  return counter;
}

}  // namespace
