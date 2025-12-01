#import <stdio.h>
#import <stdlib.h>
#import <string.h>

int main() {
  FILE *file;
  char filename[] = "example.txt";
  const char whitespace[] = " \t\n";
  char line[256]; // Buffer to store each line
  char *token;

  // Open the file in read mode
  file = fopen(filename, "r");

  // Check if the file opened successfully
  if (file == NULL) {
    perror("Error opening file");
    return EXIT_FAILURE;
  }

  printf("Contents of the file:\n");

  // Read the file line by line
  while (fgets(line, sizeof(line), file) != NULL) {
    /* printf("%s", line); // Print each line */

    token = strtok(line, whitespace);
    int number = atoi(token);
    printf("decimal %d\n", number); // Print each token
    // Loop through and extract subsequent tokens
    while (token != NULL) {
      printf("token: %s\n", token);     // Print each token
      token = strtok(NULL, whitespace); // Get the next token
      int number = atoi(token);
      printf("decimal %d\n", number); // Print each token
    }
  }

  // Close the file
  fclose(file);

  return EXIT_SUCCESS;
}
