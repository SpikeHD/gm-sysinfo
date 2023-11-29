sysinfo_init();

var print = show_debug_message;

function byte_to_readable(_bytes) {
    var _units = ["B", "KB", "MB", "GB", "TB"];
    var _unit = 0;
    while (_bytes > 1024) {
        _bytes /= 1024;
		
      if (_unit >= array_length(_units) - 1) {
        break;
      }
    
      _unit++;
    }

    return string(_bytes) + " " + _units[_unit];
}


print("sysinfo_is_initialized: " + string(sysinfo_is_initialized()));
print("sysinfo_get_username: " + sysinfo_get_username());
print("sysinfo_get_pid: " + string(sysinfo_get_pid()));
print("sysinfo_get_memory_max: " + byte_to_readable(sysinfo_get_memory_max()));
print("sysinfo_get_core_count: " + string(sysinfo_get_core_count()));
print("sysinfo_get_cpu_frequency: " + string(sysinfo_get_cpu_frequency()));
print("sysinfo_get_cpu_name: " + sysinfo_get_cpu_name());
print("sysinfo_get_cpu_brand: " + sysinfo_get_cpu_brand());
print("sysinfo_get_cpu_vendor_id: " + sysinfo_get_cpu_vendor_id());
print("sysinfo_sys_memory_used: " + byte_to_readable(sysinfo_sys_memory_used()));
print("sysinfo_proc_memory_used: " + byte_to_readable(sysinfo_proc_memory_used()));
print("sysinfo_sys_cpu_usage: " + string(sysinfo_sys_cpu_usage()));
print("sysinfo_proc_cpu_usage: " + string(sysinfo_proc_cpu_usage()));
