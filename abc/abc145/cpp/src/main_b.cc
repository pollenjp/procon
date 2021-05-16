#include <iostream>
#include <string>

bool solve(int n, std::string s) {
  if (n % 2 != 0) {
    return false;
  };
  std::string sub_str = s.substr(0, s.length() / 2);
  return sub_str == s.substr(s.length() / 2, s.length() / 2);
}

int main() {
  int N;
  std::string S;
  std::cin >> N >> S;

  std::string answer;
  if (solve(N, S)) {
    answer = "Yes";
  } else {
    answer = "No";
  }
  std::cout << answer << std::endl;
  return EXIT_SUCCESS;
}
