#include <algorithm>
#include <string>
#include <vector>
#include <map>

namespace {

std::string solve(std::string s, const int k)
{
  // int left;
  // std::map<char, int> char_map;
  // std::vector<char> char_set();
  std::sort(s.begin(), s.end());

  int cnt(1);
  if (k == cnt) {return s;}
  while(std::next_permutation(s.begin(), s.end())){
    cnt++;
    if (k == cnt) {return s;}
  }
  return "";
}
}  // namespace
