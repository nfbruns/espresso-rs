/*
  Entry point of the espresso source code.
  Replaces main.c
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "espresso.h"

// Runs plan espresso with no flags
char * run_espresso(FILE * fpla, FILE *outfile) {
  pPLA PLA;
  bool error;
  cost_t cost;
  pcover fold;
  int out_type = F_type;

  if (read_pla(fpla, TRUE, TRUE, FD_type, &PLA) == EOF) {
    return NULL;
  }

  // makes sure free() won't crash on this variable
  PLA->filename = NULL;

  fold = sf_save(PLA->F);
  PLA->F = espresso(PLA->F, PLA->D, PLA->R);
  EXECUTE(error = verify(PLA->F, fold, PLA->D), VERIFY_TIME, PLA->F, cost);

  if (error) {
    PLA->F = fold;
    (void) check_consistency(PLA);
  } else {
    free_cover(fold);
  }

  fprint_pla(outfile, PLA, out_type);

  /* cleanup all used memory */
  free_PLA(PLA);
  FREE(cube.part_size);
  setdown_cube();             /* free the cube/cdata structure data */
  sf_cleanup();               /* free unused set structures */
  sm_cleanup();               /* sparse matrix cleanup */
}

char * run_d1merge(FILE * fpla, FILE *outfile) {
  pPLA PLA;
  char * result = NULL;
  int out_type = F_type;

  int first = -1;
  int last = -1;
  int i;

  if (read_pla(fpla, TRUE, TRUE, FD_type, &PLA) == EOF) {
    return NULL;
  }

  // makes sure free() won't crash on this variable
  PLA->filename = NULL;

  if (first < 0 || first >= cube.num_vars) {
      first = 0;
  }
  if (last < 0 || last >= cube.num_vars) {
      last = cube.num_vars - 1;
  }
  for(i = first; i <= last; i++) {
      PLA->F = d1merge(PLA->F, i);
  }

  fprint_pla(outfile, PLA, out_type);

  /* cleanup all used memory */
  free_PLA(PLA);
  FREE(cube.part_size);
  setdown_cube();             /* free the cube/cdata structure data */
  sf_cleanup();               /* free unused set structures */
  sm_cleanup();               /* sparse matrix cleanup */
}

FILE *create_file_with_contents(const char *data, unsigned int length) {
  FILE *f = tmpfile();
  fwrite(data, length, 1, f);
  rewind(f);
  return f;
}

// allocates!
char *read_file_contents(FILE *f) {
    long length;
    char *buffer;

    if (fseek(f, 0, SEEK_END) != 0) {
        return NULL;
    }
    length = ftell(f);

    if (fseek(f, 0L, SEEK_SET) != 0) {
        return NULL;
    }

    /* Allocate memory to contain whole file */
    buffer = (char*) malloc(length+1);
    if (buffer == NULL) {
        return NULL;
    }

    if (fread(buffer, length, 1, f) != 1) {
        return NULL;
    }
    buffer[length] = '\0';
    return buffer;
}

char * run_espresso_from_data(const char *data, unsigned int length, char ** out) {
  if (length == 0) {
    return NULL;
  }

  FILE *tempPLA = create_file_with_contents(data, length);
  FILE *outfile = tmpfile();
  run_espresso(tempPLA, outfile);
  fclose(tempPLA);
  *out = read_file_contents(outfile);
  fclose(outfile);
}

char * run_d1merge_from_data(const char * data, unsigned int length, char ** out) {
  if (length == 0) {
    return NULL;
  }

  FILE *tempPLA = create_file_with_contents(data, length);
  FILE *outfile = tmpfile();
  run_d1merge(tempPLA, outfile);
  fclose(tempPLA);
  *out = read_file_contents(outfile);
  fclose(outfile);
}
