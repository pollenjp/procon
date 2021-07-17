
#include <string>

namespace {

std::string solve(const int N, const std::string S) {
  std::string::size_type i;
  for (i = 0; i < S.size(); i++) {
    if (S[i] == '1') {
      break;
    }
  }

  if (i % 2 == 0) {
    // 高橋くんが負けたので
    return std::string("Takahashi");
  } else {
    return std::string("Aoki");
  }
}
}  // namespace
