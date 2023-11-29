use std::ffi::CString;
use std::os::raw::c_char;
use sysinfo::{CpuExt, CpuRefreshKind, Pid, PidExt, ProcessExt, System, SystemExt};

mod gpu;

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

    // trigger a refresh
    SYSTEM.as_mut().unwrap().refresh_all();

    // After a short amount of time, trigger a CPU refresh
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL * 2);
    SYSTEM
      .as_mut()
      .unwrap()
      .refresh_cpu_specifics(CpuRefreshKind::everything());
  }
}

// Exe path to name
#[no_mangle]
fn exe_path_to_name(path: &str) -> String {
  let path = path.replace('\\', "/");

  let path = path.split('/').collect::<Vec<&str>>();

  // Get rid of extension if it exists
  let path = path[path.len() - 1].split('.').collect::<Vec<&str>>();

  path[0].to_string()
}

// Get username
#[no_mangle]
pub extern "C" fn get_username() -> *mut c_char {
  let username = whoami::username();

  CString::new(username)
    .unwrap_or(CString::new("").unwrap())
    .into_raw()
}

// Get hostname
#[no_mangle]
pub extern "C" fn get_hostname() -> *mut c_char {
  let hostname = whoami::hostname();

  CString::new(hostname)
    .unwrap_or(CString::new("").unwrap())
    .into_raw()
}

// Get pid as float
#[no_mangle]
pub extern "C" fn get_pid() -> f64 {
  std::process::id() as f64
}

// Get memory maximum for the system
#[no_mangle]
pub extern "C" fn get_memory_max() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe { SYSTEM.as_mut().unwrap().total_memory() as f64 }
}

// Get core count
#[no_mangle]
pub extern "C" fn get_core_count() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe { SYSTEM.as_mut().unwrap().cpus().len() as f64 }
}

// Get CPU frequency
#[no_mangle]
pub extern "C" fn get_cpu_frequency() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe {
    // CPU data should always have a refresh_cpu() call before it
    SYSTEM
      .as_mut()
      .unwrap()
      .refresh_cpu_specifics(CpuRefreshKind::everything());

    let cpus = SYSTEM.as_mut().unwrap().cpus();

    cpus[0].frequency() as f64
  }
}

// Get CPU name
#[no_mangle]
pub extern "C" fn get_cpu_name() -> *mut c_char {
  if !is_initialized() {
    eprintln!("System not initialized!");
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
    eprintln!("System not initialized!");
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
    eprintln!("System not initialized!");
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

// Get GPU name
#[no_mangle]
pub extern "C" fn get_gpu_name() -> *mut c_char {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return CString::new("").unwrap().into_raw();
  }

  unsafe {
    CString::new(gpu::get_gpu_name())
      .unwrap_or(CString::new("").unwrap())
      .into_raw()
  }
}

#[no_mangle]
pub extern "C" fn get_gpu_vram() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe { gpu::get_gpu_vram() as f64 }
}

// Get memory used for the system
#[no_mangle]
pub extern "C" fn sys_memory_used() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  // refresh
  unsafe {
    SYSTEM.as_mut().unwrap().refresh_memory();
  }

  unsafe { SYSTEM.as_mut().unwrap().used_memory() as f64 }
}

// Get memory used for the process
#[no_mangle]
pub extern "C" fn proc_memory_used() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe {
    // Refresh and get the process
    SYSTEM.as_mut().unwrap().refresh_processes();

    // Safe to unwrap, we already check if the system is initialized (twice!)
    let pid = Pid::from_u32(get_pid() as u32);
    let proc = SYSTEM.as_mut().unwrap().process(pid).unwrap();

    proc.memory() as f64
  }
}

// Get the CPU usage for the system
#[no_mangle]
pub extern "C" fn sys_cpu_usage() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  // Refresh
  unsafe {
    SYSTEM.as_mut().unwrap().refresh_cpu();
  }

  unsafe {
    let cpus = SYSTEM.as_mut().unwrap().cpus();

    let mut total = 0.0;

    for cpu in cpus {
      total += cpu.cpu_usage() as f64;
    }

    if total.is_nan() {
      return 0.0;
    }

    total
  }
}

// Get the CPU usage for the process
#[no_mangle]
pub extern "C" fn proc_cpu_usage() -> f64 {
  if !is_initialized() {
    eprintln!("System not initialized!");
    return -1.0;
  }

  unsafe {
    // CPU data should always have a refresh_cpu() call before it
    SYSTEM
      .as_mut()
      .unwrap()
      .refresh_cpu_specifics(CpuRefreshKind::everything());

    // Refresh and get the process
    SYSTEM.as_mut().unwrap().refresh_processes();

    // Safe to unwrap, we already check if the system is initialized (twice!)
    let pid = Pid::from_u32(get_pid() as u32);
    let proc = SYSTEM.as_mut().unwrap().process(pid).unwrap();

    proc.cpu_usage() as f64
  }
}
