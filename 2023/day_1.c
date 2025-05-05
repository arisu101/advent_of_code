#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TEST_FILE "test_day1.txt"

char *read_from_file(char *filename) {
  FILE *file = fopen(filename, "r");
  fseek(file, 0, SEEK_END);
  int length = ftell(file);
  fseek(file, 0, SEEK_SET);
  char *content = malloc(sizeof(char) * length + 1);
  fread(content, 1, length, file);
  content[length] = '\0';
  printf("%s\n", content);
  return content;
}

typedef struct {
  char num[2];
  bool is_initted;
} Nums;

int main(int argc, char *argv[]) {
  char *content = read_from_file(TEST_FILE);
  char first_num[2];
  char last_num[2];
  for (size_t i = 0; content[i] != '\0'; i++) {
    if (content[i] == '\n') {
    }
    if (isdigit(content[i])) {
      printf("%c\n", content[i]);
    }
  }
  return EXIT_SUCCESS;
}
