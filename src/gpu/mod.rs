#[cfg_attr(target_os = "macos", path = "apple.rs")]
#[cfg_attr(not(target_os = "macos"), path = "other.rs")]
mod info;

pub unsafe fn get_gpu_name() -> String {
  info::get_gpu_name()
}

pub unsafe fn get_gpu_vram() -> u64 {
  info::get_gpu_vram()
}
