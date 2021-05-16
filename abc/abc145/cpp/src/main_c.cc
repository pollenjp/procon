#include <cmath>
#include <iomanip>  // setprecision
#include <iostream>
#include <string>
#include <vector>

class Point2D {
 public:
  int x, y;

  Point2D() : x(0), y(0) {}  // 初期化は何もしない

  Point2D(int x0, int y0) : x(x0), y(y0) {}

  Point2D operator+(const Point2D &p0) const {
    Point2D p1;
    p1.x = this->x + p0.x;
    p1.y = this->y + p0.y;
    return p1;
  }

  static double length(const Point2D &p0, const Point2D &p1) {
    double x, y;
    x = static_cast<double>(p0.x - p1.x);
    y = static_cast<double>(p0.y - p1.y);
    return std::sqrt(std::pow(x, 2) + std::pow(y, 2));
  }
  double length(const Point2D &p0) const { return Point2D::length(p0, *this); }
};

double solve(int num_vertices, std::vector<Point2D> vertex_vector) {
  double total_length_of_all_edges(0);
  Point2D a_point = vertex_vector.at(0);
  for (auto iter1 = vertex_vector.begin(); iter1 != vertex_vector.end();
       iter1++) {
    for (auto iter2 = iter1 + 1; iter2 != vertex_vector.end(); iter2++) {
      if (iter1 != iter2) {
        total_length_of_all_edges += (*iter1).length(*iter2);
      }
    }
  }
  return total_length_of_all_edges * 2.0 / num_vertices;
}

int main() {
  int N;
  std::cin >> N;
  std::vector<Point2D> vertex_vector;

  for (int i = 0; i < N; i++) {
    int x, y;
    std::cin >> x >> y;
    vertex_vector.emplace_back(Point2D(x, y));
  }

  std::cout << std::fixed << std::setprecision(15) << solve(N, vertex_vector)
            << std::endl;

  return EXIT_SUCCESS;
}
