
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

// #define DEBUG
#include "./aoc.h"

const bool do_solution = true;

int line_to_instruction(char *line);
int *instructions_from_file(char *filename, int *len);

int part1(int *instructions, size_t len) {
  // 1150
  int dial = 50;
  int num_zeros = 0;
  for (size_t i = 0; i < len; i++) {
    dial += instructions[i];
    dial %= 100;
    if (dial == 0) {
      num_zeros += 1;
    }
  }
  return num_zeros;
}

// 6738, have 6634
int part2(int *instructions, size_t len) {
  int dial = 50;
  int count = 0;
  for (size_t i = 0; i < len; i++) {

    // check if we started at zero so we dont count
    // it when turning left initally
    bool started_at_zero = dial == 0;

    // get the turn direction instruction
    int instr = instructions[i];
    // turn the dial
    dial += instr;

    // reconsile oob values, each loop is passing over zero
    while (dial > 100 || dial < 0) {
      while (dial >= 100) {
        dial -= 100;
        count += 1;
      }
      if (dial < 0) {
        dial += 100;
        if (!started_at_zero) {
          count += 1;
        }
        started_at_zero = false;
      }
    }
    // reset if we are at 100
    dial %= 100;
    // landed on zero after all the turning
    if (dial == 0) {
      count += 1;
    }
  }
  return count;
}

void test_solve_part1(void) {
  int len = 0;
  int *instructions = instructions_from_file("test.txt", &len);
  int answer = part1(instructions, len);
  ASSERT_WITH_MSG(answer == 3, "got %d, expected 3", answer);
}
void test_solve_part2(void) {
  int len = 0;
  int *instructions = instructions_from_file("test.txt", &len);
  int answer = part2(instructions, len);
  ASSERT_WITH_MSG(answer == 6, "got %d, expected 6", answer);
}

int main(void) {
  test_solve_part1();
  test_solve_part2();

  if (do_solution) {
    int len = 0;
    int *instructions = instructions_from_file("input.txt", &len);

    printf("part1: %d\n", part1(instructions, len));

    printf("part2: %d\n", part2(instructions, len));
  }

  return 0;
}

int line_to_instruction(char *line) {
  int sign = (line[0] == 'L') ? -1 : 1;
  long amount = strtol(line + 1, NULL, 10);
  return sign * (int)amount;
}

int *instructions_from_file(char *filename, int *len) {
  FILE *file = fopen(filename, "r");
  if (file == NULL) {
    printf("Error: Could not open file\n");
    return NULL;
  }

  int *instructions = NULL;
  int capacity = 100;
  int count = 0;
  instructions = (int *)malloc(capacity * sizeof(int));
  if (instructions == NULL) {
    printf("Error: Memory allocation failed\n");
    fclose(file);
    return NULL;
  }

  char line[256];
  while (fgets(line, sizeof(line), file) != NULL) {
    if (count >= capacity) {
      capacity *= 2; // Double the capacity
      instructions = (int *)realloc(instructions, capacity * sizeof(int));
      if (instructions == NULL) {
        printf("Error: Memory reallocation failed\n");
        fclose(file);
        return NULL;
      }
    }
    int instr = line_to_instruction(line);
    instructions[count++] = instr;
  }
  *len = count;

  fclose(file);
  return instructions;
}
