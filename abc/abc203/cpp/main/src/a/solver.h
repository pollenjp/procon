
namespace {
int solve(int a, int b, int c) {
  if (a == b) {
    return c;
  } else if (b == c) {
    return a;
  } else if (c == a) {
    return b;
  }
  return 0;
}
}  // namespace
