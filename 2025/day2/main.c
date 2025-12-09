#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #define DEBUG
#include "aoc.h"

int *read(const char *filename, int *len);
char **str_split(char *a_str, const char a_delim);

bool bad_id(int num) {
  int digits = (int)log10(num) + 1;

  // patterns can only be up to half the length of the number
  digits /= 2;

  int *pattern = malloc(digits * sizeof *pattern);
  if (!pattern)
    return 0;

  int place = (int)pow(10, digits); /* 1000 for 9908 */
  int prefix = 0;
  int max = 0;
  for (int i = 0; i <= digits; ++i) {
    int leading = num / place; /* current most-significant digit */
    prefix = prefix * 10 + leading;
    pattern[i] = prefix;
    num %= place; /* remove that digit */
    place /= 10;
    max = i;
  }
  println("pattern");
  for (int i = 0; i < max; ++i)
    printf("%d ", pattern[i]); /* 9 99 990 9908 */
  putchar('\n');
  return false;
}
int part1(int *ids, int groups) {

  int bad_id_idx = 0;
  int *bad_ids = malloc(sizeof(int) * 256);
  // TODO: i = 0!!!!!
  for (int i = 4; i < groups; i += 2) {
    printf("%d-%d (%d)\n", ids[i], ids[i + 1], ids[i + 1] - ids[i]);
    for (int j = ids[i]; j <= ids[i + 1]; ++j) {
      if (bad_id(j)) {
        bad_ids[bad_id_idx++] = j;
      }
      break;
    }
    break;
  }
  return 0;
}

int part2(int *UNUSED(lines), size_t UNUSED(len)) { return 0; }

int part1_example(void) {
  const int p1_answer = 0;

  int len = 0;
  int *ids = read("./example.txt", &len);
  if (!ids) {
    return 1;
  }
  int answer = part1(ids, len);

  ASSERT_WITH_MSG(answer == p1_answer, "got %d, expected %d", answer,
                  p1_answer);

  free(ids);
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

// int index(int x, int y, int max_x) {
//     return y * max_x + x;
// }

int get_ids(const char *line, int *out, int *ngroups) {

  char **tokens = str_split((char *)line, ',');
  if (tokens) {
    for (int i = 0; *(tokens + i); i++) {
      // println("%s", *(tokens + i));
      char **ids = str_split(*(tokens + i), '-');
      for (int j = 0; *(ids + j); j++) {
        // println("%s", *(ids + j));
        out[*ngroups] = strtol(*(ids + j), NULL, 10);
        ++(*ngroups);
        free(*(ids + j));
      }
      free(*(tokens + i));
    }
    free(tokens);
    return 0;
  }
  return 1;
}

int *read(const char *filename, int *nrows) {
  FILE *fp = fopen(filename, "r");
  if (!fp) {
    println("failed to open file");
    perror("fopen");
    return NULL;
  }

  // prealloc 512, given the input this will be plenty
  size_t cap = 512;
  int *ids = malloc(cap * sizeof *ids);
  if (!ids) {
    println("failed malloc of ids");
    fclose(fp);
    return NULL;
  }

  int groups = 0;
  char line[512];
  // single line in input
  fgets(line, sizeof line, fp);
  // remove extra newline if there
  if (line[strlen(line) - 1] == '\n') {
    line[strlen(line) - 1] = 0;
  }

  if (get_ids(line, ids, &groups)) {
    free(ids);
    fclose(fp);
    return NULL;
  }
  fclose(fp);

  ids = realloc(ids, groups * sizeof *ids);
  *nrows = groups;
  return ids;
}

// https://stackoverflow.com/questions/9210528/split-string-with-delimiters-in-c
// while ((token = strsep(&str, ","))) my_fn(token);
char **str_split(char *a_str, const char a_delim) {
  char **result = 0;
  size_t count = 0;
  char *tmp = a_str;
  char *last_comma = 0;
  char delim[2];
  delim[0] = a_delim;
  delim[1] = 0;

  /* Count how many elements will be extracted. */
  while (*tmp) {
    if (a_delim == *tmp) {
      count++;
      last_comma = tmp;
    }
    tmp++;
  }

  /* Add space for trailing token. */
  count += last_comma < (a_str + strlen(a_str) - 1);

  /* Add space for terminating null string so caller
     knows where the list of returned strings ends. */
  count++;

  result = malloc(sizeof(char *) * count);

  if (result) {
    size_t idx = 0;
    char *token = strtok(a_str, delim);

    while (token) {
      assert(idx < count);
      *(result + idx++) = strdup(token);
      token = strtok(0, delim);
    }
    assert(idx == count - 1);
    *(result + idx) = 0;
  }

  return result;
}
