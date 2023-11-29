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

extern const char* get_username();

extern double get_pid();
extern double get_memory_max();
extern double get_core_count();

extern double get_cpu_frequency();
extern const char* get_cpu_name();
extern const char* get_cpu_brand();
extern const char* get_cpu_vendor_id();

extern double sys_memory_used();
extern double proc_memory_used();
extern double sys_cpu_usage();
extern double proc_cpu_usage();

int main() {
  // Test is_initialized
  printf("is_initialized: %d\n", is_initialized());
  assert(!is_initialized());

  // Test init
  init();
  printf("is_initialized (after init): %d\n", is_initialized());
  assert(is_initialized());

  // Test get_pid
  printf("PID: %f\n", get_pid());
  assert(get_pid() > 0);

  // Test get_core_count
  printf("Core count: %f\n", get_core_count());
  assert(get_core_count() > 0);

  // Test get_memory_max
  printf("Memory max: %f\n", get_memory_max());
  assert(get_memory_max() > 0);

  // Test get_cpu_frequency
  printf("CPU frequency: %f\n", get_cpu_frequency());
  assert(get_cpu_frequency() > 0);

  // Test get_cpu_name
  printf("CPU name: %s\n", get_cpu_name());
  assert(strlen(get_cpu_name()) > 0);

  // Test get_cpu_brand
  printf("CPU brand: %s\n", get_cpu_brand());
  assert(strlen(get_cpu_brand()) > 0);

  // Test get_cpu_vendor_id
  printf("CPU vendor ID: %s\n", get_cpu_vendor_id());
  assert(strlen(get_cpu_vendor_id()) > 0);

  // Test sys_memory_used
  printf("System memory used: %f\n", sys_memory_used());
  assert(sys_memory_used() > 0);

  // Test proc_memory_used
  printf("Process memory used: %f\n", proc_memory_used());
  assert(proc_memory_used() > 0);

  // Test sys_cpu_usage
  printf("System CPU usage: %f\n", sys_cpu_usage());
  assert(sys_cpu_usage() > 0);

  // Test proc_cpu_usage
  printf("Process CPU usage: %f\n", proc_cpu_usage());
  assert(proc_cpu_usage() > 0);

  printf("All tests passed!\n");

  return 0;
}