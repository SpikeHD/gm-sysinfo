<div align="center">
  <h1>GM Sysinfo</h1>
  <p>
    Profile your game, and get it's (and the systems) resource usage!
  </p>
</div>

# Examples

Below are some basic examples of the functionality in this extension

## Display memory usage
```gml
// Get max memory of the system
var max_mem = sysinfo_sys_mem_max();

// Get the memory usage of the game
var mem = sysinfo_mem_usage();

// Display the cur / max (pct%)
var max_mb = max_mem / 1024 / 1024;
var mem_mb = mem / 1024 / 1024;

draw_text(0, 0, string(mem_mb) + " / " + string(max_mb) + " (" + string(mem / max_mem * 100) + "%)");
```

## Display CPU usage
```gml
// Get the CPU and memory usage of the game
var cpu_name = sysinfo_cpu_name();
var cpu = sysinfo_cpu_usage();

// Display the %
draw_text(0, 0, cpu_name + ": " + string(cpu) + "%");
```

# TODO

- [ ] CPU usage
- [ ] Memory usage
- [ ] System name/hostname
- [ ] GPU usage(?)

# Documentation