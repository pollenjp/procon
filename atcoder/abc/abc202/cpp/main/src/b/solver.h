#pragma once

#include <string>

namespace {

std::string solve(const std::string &str) {
  std::string ret_value(str.size(), '0');
  char c;
  for (std::string::size_type i = 0; i < str.size(); i++) {
    switch (str[i]) {
      case '6':
        c = '9';
        break;
      case '9':
        c = '6';
        break;
      default:
        c = str[i];
        break;
    }
    ret_value[str.size() - 1 - i] = c;
  }
  return ret_value;
}

}  // namespace
