#[cfg(not(target_os = "macos"))]
use rust_gpu_tools::Device;

#[cfg(target_os = "macos")]
use metal::Device;

#[cfg(not(target_os = "macos"))]
pub fn get_gpu_name() -> String {
  let gpus = Device::all();

  gpus[0].name().to_string()
}

#[cfg(not(target_os = "macos"))]
pub fn get_gpu_vram() -> u64 {
  let gpus = Device::all();

  gpus[0].memory()
}

#[cfg(target_os = "macos")]
pub fn get_gpu_name() -> String {
  let gpus = Device::all();

  if gpus.is_empty() {
    return String::from("N/A");
  }

  gpus[0].name().to_string()
}

#[cfg(target_os = "macos")]
pub fn get_gpu_vram() -> u64 {
  let gpus = Device::all();

  if gpus.is_empty() {
    return 0;
  }

  gpus[0].recommended_max_working_set_size()
}
