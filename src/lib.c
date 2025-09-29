#include "lib.h"

uint64_t factorial(const uint64_t n) {
  if (n <= 1) {
    return 1;
  }

  return n * factorial(n - 1);
}

int64_t add(const int64_t a, const int64_t b) { return a + b; }
