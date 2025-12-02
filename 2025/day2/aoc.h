#include <stdio.h>
#include <stdlib.h>

#define ASSERT_WITH_MSG(condition, fmt, ...)                                   \
  do {                                                                         \
    if (!(condition)) {                                                        \
      char msg[256];                                                           \
      sprintf(msg, fmt, __VA_ARGS__);                                          \
      printf("Assertion failed: %s\n", msg);                                   \
      assert(0);                                                               \
    }                                                                          \
  } while (0)

#ifdef DEBUG
#define DEBUGF(...) printf(__VA_ARGS__)
#else
#define DEBUGF(...)                                                            \
  do {                                                                         \
  } while (0)
#endif

#ifdef __GNUC__
#define UNUSED(x) UNUSED_##x __attribute__((__unused__))
#else
#define UNUSED(x) UNUSED_##x
#endif
