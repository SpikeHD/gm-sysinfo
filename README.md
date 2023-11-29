<div align="center">
  <h1>GM Sysinfo</h1>
  <p>
    Cross-platform GameMaker extension for getting system information and resource usage 
  </p>
</div>

# Table of Contents
* [Table of Contents](#table-of-contents)
* [Examples](#examples)
  * [Display memory usage](#display-memory-usage)
  * [Display CPU usage](#display-cpu-usage)
  * [Get the host/username](#get-the-hostusername)
* [TODO](#todo)
* [Documentation](#documentation)

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
- [x] Memory usage
- [x] System name/hostname
- [ ] GPU usage(?)

# Documentation