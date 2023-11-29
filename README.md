<div align="center">
  <h1>GM Sysinfo</h1>
  <p>
    Cross-platform GameMaker extension for getting system information and resource usage 
  </p>
  <img width="70%" src="https://github.com/SpikeHD/gm-sysinfo/assets/25207995/27bbeb40-f1db-4fd1-b2b1-3790f67aeaaf" />
</div>


# Table of Contents
* [Table of Contents](#table-of-contents)
* [Examples](#examples)
  * [Display memory usage](#display-memory-usage)
  * [Display CPU usage](#display-cpu-usage)
  * [Get the host/username](#get-the-hostusername)
* [TODO](#todo)
* [Documentation](#documentation)
  * [General](#general)
    * [`sysinfo_init()`](#sysinfo_init)
    * [`sysinfo_get_username()`](#sysinfo_get_username)
    * [`sysinfo_get_hostname()`](#sysinfo_get_hostname)
    * [`sysinfo_get_pid()`](#sysinfo_get_pid)
  * [CPU](#cpu)
    * [`sysinfo_get_cpu_usage()`](#sysinfo_get_cpu_usage)
    * [`sysinfo_get_cpu_brand()`](#sysinfo_get_cpu_brand)
    * [`sysinfo_get_cpu_cores()`](#sysinfo_get_cpu_cores)
    * [`sysinfo_get_cpu_frequency()`](#sysinfo_get_cpu_frequency)
    * [`sysinfo_sys_cpu_usage()`](#sysinfo_sys_cpu_usage)
    * [`sysinfo_proc_cpu_usage()`](#sysinfo_proc_cpu_usage)
    * [`sysinfo_get_core_count()`](#sysinfo_get_core_count)
    * [`sysinfo_get_cpu_vendor_id()`](#sysinfo_get_cpu_vendor_id)
  * [GPU](#gpu)
    * [`sysinfo_get_gpu_name()`](#sysinfo_get_gpu_name)
    * [`sysinfo_get_gpu_vram()`](#sysinfo_get_gpu_vram)
  * [Memory](#memory)
    * [`sysinfo_get_memory_max()`](#sysinfo_get_memory_max)
    * [`sysinfo_sys_memory_used()`](#sysinfo_sys_memory_used)
    * [`sysinfo_proc_memory_used()`](#sysinfo_proc_memory_used)

# Examples

Below are some basic examples of the functionality in this extension

## Display memory usage
```javascript
// Init
sysinfo_init();

// Get max memory of the system
var max_mem = sysinfo_sys_mem_max();

// Get the memory usage of the game
var mem = sysinfo_proc_mem_usage();

// Display the cur / max (pct%)
var max_mb = max_mem / 1024 / 1024;
var mem_mb = mem / 1024 / 1024;

draw_text(0, 0, string(mem_mb) + " / " + string(max_mb) + " (" + string(mem / max_mem * 100) + "%)");
```

## Display CPU usage
```javascript
// Init
sysinfo_init();

// Get the CPU and memory usage of the game
var cpu_name = sysinfo_get_cpu_brand();
var cpu = sysinfo_get_cpu_usage();

// Display the %
draw_text(0, 0, cpu_name + ": " + string(cpu) + "%");
```

## Get the host/username
```javascript
// Init
sysinfo_init();

// Get the host name
var host = sysinfo_get_hostname();
var user = sysinfo_get_username();

// Display the host name
draw_text(0, 0, user + "@" + host);
```

# TODO

- [x] CPU usage
  - [ ] Get usage for a specific core
- [x] Memory usage
- [x] System name/hostname
- [ ] GPU usage(?)

# Documentation

## General

### `sysinfo_init()`
Initializes the extension. This must be called before any other functions are called.

### `sysinfo_get_username()`
Returns the username of the current user.

### `sysinfo_get_hostname()`
Returns the hostname of the system.

### `sysinfo_get_pid()`
Returns the PID of the game.

## CPU

### `sysinfo_get_cpu_usage()`
Returns the current CPU usage of the game in percent.

### `sysinfo_get_cpu_brand()`
Returns the brand name of the CPU.

### `sysinfo_get_cpu_cores()`
Returns the number of cores the CPU has.

### `sysinfo_get_cpu_frequency()`
Returns the frequency of the CPU in MHz.

### `sysinfo_sys_cpu_usage()`
Returns the CPU usage of the system in percent.

### `sysinfo_proc_cpu_usage()`
Returns the CPU usage of the game in percent.

### `sysinfo_get_core_count()`
Returns the number of cores the CPU has.

### `sysinfo_get_cpu_vendor_id()`
Returns the vendor ID of the CPU.

## GPU

### `sysinfo_get_gpu_name()`
Returns the name of the GPU.

### `sysinfo_get_gpu_vram()`
Returns the amount of VRAM the GPU has in bytes.

## Memory

### `sysinfo_get_memory_max()`
Returns the maximum memory of the system in bytes.

### `sysinfo_sys_memory_used()`
Returns the memory used by the system in bytes.

### `sysinfo_proc_memory_used()`
Returns the memory used by the game in bytes.

