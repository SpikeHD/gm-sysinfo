use std::ffi::CString;
use std::os::raw::c_char;
use sysinfo::{CpuExt, Pid, PidExt, Process, ProcessExt, System, SystemExt, User, UserExt};

static mut SYSTEM: Option<System> = None;

// Check if initialized
#[no_mangle]
pub extern "C" fn is_initialized() -> bool {
    unsafe { SYSTEM.is_some() }
}

// Initializes the System object
#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        SYSTEM = Some(System::new());
    }
}

// Get the current exe, and get the PID from that exe
fn get_pid_internal() -> Result<Pid, Box<dyn std::error::Error>> {
    if !is_initialized() {
        // Error
        return Err("System not initialized".into());
    }

    unsafe {
        let exe = std::env::current_exe().unwrap();
        let exe = exe.to_str().unwrap();

        // Refresh the system processes
        SYSTEM.as_mut().unwrap().refresh_processes();

        let proc_with_name = SYSTEM.as_mut().unwrap().processes_by_name(exe);
        let proc_with_name = proc_with_name.collect::<Vec<&Process>>();

        // Get the first process with the name of the current exe
        let proc_with_name = proc_with_name[0];

        // Get the PID of the process
        Ok(proc_with_name.pid())
    }
}

// Get Pid as number
#[no_mangle]
pub extern "C" fn get_pid() -> f64 {
    get_pid_internal().unwrap_or(Pid::from(0)).as_u32() as f64
}

// Get memory maximum for the system
#[no_mangle]
pub extern "C" fn get_memory_max() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe { SYSTEM.as_mut().unwrap().total_memory() as f64 }
}

// Get CPU frequency
#[no_mangle]
pub extern "C" fn get_cpu_frequency() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe {
        let cpus = SYSTEM.as_mut().unwrap().cpus();

        cpus[0].frequency() as f64
    }
}

// Get CPU name
#[no_mangle]
pub extern "C" fn get_cpu_name() -> *mut c_char {
    if !is_initialized() {
        return CString::new("").unwrap().into_raw();
    }

    unsafe {
        let cpu_info = SYSTEM.as_mut().unwrap().global_cpu_info();
        let cpu_name = cpu_info.name();

        CString::new(cpu_name)
            .unwrap_or(CString::new("").unwrap())
            .into_raw()
    }
}

// CPU Brand
#[no_mangle]
pub extern "C" fn get_cpu_brand() -> *mut c_char {
    if !is_initialized() {
        return CString::new("").unwrap().into_raw();
    }

    unsafe {
        let cpu_info = SYSTEM.as_mut().unwrap().global_cpu_info();
        let cpu_brand = cpu_info.brand();

        CString::new(cpu_brand)
            .unwrap_or(CString::new("").unwrap())
            .into_raw()
    }
}

// Get CPU vendor ID
#[no_mangle]
pub extern "C" fn get_cpu_vendor_id() -> *mut c_char {
    if !is_initialized() {
        return CString::new("").unwrap().into_raw();
    }

    unsafe {
        let cpu_info = SYSTEM.as_mut().unwrap().global_cpu_info();
        let cpu_vendor_id = cpu_info.vendor_id();

        CString::new(cpu_vendor_id)
            .unwrap_or(CString::new("").unwrap())
            .into_raw()
    }
}

// Get memory used for the system
#[no_mangle]
pub extern "C" fn sys_memory_used() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe { SYSTEM.as_mut().unwrap().used_memory() as f64 }
}

// Get memory used for the process
#[no_mangle]
pub extern "C" fn proc_memory_used() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe {
        // Refresh and get the process
        SYSTEM.as_mut().unwrap().refresh_processes();

        // Safe to unwrap, we already check if the system is initialized (twice!)
        let pid = get_pid_internal().unwrap();
        let proc = SYSTEM.as_mut().unwrap().process(pid).unwrap();

        proc.memory() as f64
    }
}

// Get the CPU usage for the system
#[no_mangle]
pub extern "C" fn sys_cpu_usage() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe {
        let cpus = SYSTEM.as_mut().unwrap().cpus();

        let mut total = 0.0;

        for cpu in cpus {
            total += cpu.cpu_usage() as f64;
        }

        total
    }
}

// Get the CPU usage for the process
#[no_mangle]
pub extern "C" fn proc_cpu_usage() -> f64 {
    if !is_initialized() {
        return -1.0;
    }

    unsafe {
        // Refresh and get the process
        SYSTEM.as_mut().unwrap().refresh_processes();

        // Safe to unwrap, we already check if the system is initialized (twice!)
        let pid = get_pid_internal().unwrap();
        let proc = SYSTEM.as_mut().unwrap().process(pid).unwrap();

        proc.cpu_usage() as f64
    }
}
