#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

// #define DEBUG
#include "./aoc.h"

#define LEVELS 5

int (*read(const char *filename, int *len))[LEVELS];

int part1(int (*lines)[LEVELS], size_t len) {

  for (int i = 0; i < (int)len; ++i) {
    for (int j = 0; j < LEVELS; ++j) {
      printf("%d ", lines[i][j]);
    }
    putchar('\n');
  }
  return 0;
}

int part2(int *UNUSED(lines), size_t UNUSED(len)) { return 0; }

int part1_example(void) {
  const int p1_answer = 0;

  auto len = 0;
  int(*levels)[LEVELS] = read("./test.txt", &len);
  if (!levels) {
    return 1;
  }
  auto answer = part1(levels, len);

  ASSERT_WITH_MSG(answer == p1_answer, "got %d, expected %d", answer,
                  p1_answer);

  free(levels);
  return 0;
}

int main(void) {
  if (part1_example()) {
    perror("something went wrong in p1_example");
    return 1;
  }

  // auto len = 0;
  // auto *lines = read("input.txt", &len);
  //
  // println("part1: %d", part1(lines, len));
  // println("part2: %d", part2(lines, len));

  return 0;
}

void convert_line(const char *line, int out[LEVELS]) {
  for (int i = 0; i < LEVELS; ++i) {
    out[i] = strtol(line, (char **)&line, 10);
  }
}

int (*read(const char *filename, int *nrows))[LEVELS] {
  FILE *fp = fopen(filename, "r");
  if (!fp) {
    println("failed to open file");
    perror("fopen");
    return NULL;
  }

  size_t cap = 16;
  int(*block)[LEVELS] = malloc(cap * sizeof *block);
  if (!block) {
    println("failed malloc of blocks");
    fclose(fp);
    return NULL;
  }

  size_t idx = 0;
  char line[256];
  while (fgets(line, sizeof line, fp)) {
    if (idx == cap) {
      cap *= 2;
      void *tmp = realloc(block, cap * sizeof *block);
      if (!tmp) {
        free(block);
        fclose(fp);
        return NULL;
      }
      block = tmp;
    }

    convert_line(line, block[idx]);

    ++idx;
  }
  fclose(fp);

  block = realloc(block, idx * sizeof *block);
  *nrows = idx;
  return block;
}
