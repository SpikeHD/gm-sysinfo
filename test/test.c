#include <stdio.h>
#include <assert.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

// Include winbase on Windows
#include <stdio.h>  /* defines FILENAME_MAX */
#ifdef _WIN32
#include <direct.h>
#define GetCurrentDir _getcwd
#else
#include <unistd.h>
#define GetCurrentDir getcwd
#endif

// Function externs for Rust functions
extern bool is_initialized();
extern void init();

extern float get_pid();
extern float get_memory_max();

extern float get_cpu_frequency();
extern const char* get_cpu_name();
extern const char* get_cpu_brand();
extern const char* get_cpu_vendor_id();

extern float sys_memory_used();
extern float proc_memory_used();
extern float sys_cpu_usage();
extern float proc_cpu_usage();

int main() {
  // Test is_initialized
  assert(!is_initialized());

  // Test init
  init();
  assert(is_initialized());

  // Test get_pid
  assert(get_pid() > 0);

  // Test get_memory_max
  assert(get_memory_max() > 0);

  // Test get_cpu_frequency
  assert(get_cpu_frequency() > 0);

  // Test get_cpu_name
  assert(strlen(get_cpu_name()) > 0);

  // Test get_cpu_brand
  assert(strlen(get_cpu_brand()) > 0);

  // Test get_cpu_vendor_id
  assert(strlen(get_cpu_vendor_id()) > 0);

  // Test sys_memory_used
  assert(sys_memory_used() > 0);

  // Test proc_memory_used
  assert(proc_memory_used() > 0);

  // Test sys_cpu_usage
  assert(sys_cpu_usage() > 0);

  // Test proc_cpu_usage
  assert(proc_cpu_usage() > 0);

  printf("All tests passed!\n");

  return 0;
}