#[cfg_attr(target_os = "macos", path = "apple.rs")]
#[cfg_attr(not(target_os = "macos"), path = "other.rs")]
mod gpu;

pub unsafe fn get_gpu_name() -> String {
  gpu::get_gpu_name()
}

pub unsafe fn get_gpu_vram() -> u64 {
  gpu::get_gpu_vram()
}