namespace {

int solve(const int n) {
  if (n <= 125) {
    return 4;
  } else if (n <= 211) {
    return 6;
  } else {
    return 8;
  }
}
}  // namespace
