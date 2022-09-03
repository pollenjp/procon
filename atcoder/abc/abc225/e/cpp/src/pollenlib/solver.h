#include <glog/logging.h>

#include <algorithm>
#include <string>
#include <utility>
#include <vector>

namespace atcoder {

namespace abc225 {

namespace e {

template <class T>
class Fraction {
 private:
  T numerator_;
  T denominator_;

 public:
  Fraction(T numerator = static_cast<T>(0), T denominator = static_cast<T>(1))
      : numerator_(numerator), denominator_(denominator) {}

  std::string String() const { return std::to_string(numerator_) + "/" + std::to_string(denominator_); }

  bool operator<(const Fraction<T> &rhs) const {
    return this->numerator_ * rhs.denominator_ < rhs.numerator_ * this->denominator_;
  }

  bool operator<=(const Fraction<T> &rhs) const {
    return this->numerator_ * rhs.denominator_ <= rhs.numerator_ * this->denominator_;
  }
};

int64_t solve(std::vector<std::pair<int64_t, int64_t>> const &xy_list) {
  std::vector<std::pair<Fraction<int64_t>, Fraction<int64_t>>> end_start_list(xy_list.size());

  for (std::size_t i(0); i < xy_list.size(); i++) {
    end_start_list[i] = std::make_pair(Fraction<int64_t>(xy_list[i].second, xy_list[i].first - 1),   // start
                                       Fraction<int64_t>(xy_list[i].second - 1, xy_list[i].first));  // end
  }

  std::sort(end_start_list.begin(), end_start_list.end());

  int64_t cnt(0);
  Fraction<int64_t> current_end;

  for (auto end_start_pair : end_start_list) {
    LOG(INFO) << "current_end=" << current_end.String() << ", start=" << end_start_pair.second.String()
              << ", end=" << end_start_pair.first.String();
    LOG(INFO) << (current_end <= end_start_pair.second);
    if (current_end <= end_start_pair.second) {
      cnt++;
      current_end = end_start_pair.first;
    }
  }

  return cnt;
}

}  // namespace e

}  // namespace abc225

}  // namespace atcoder
