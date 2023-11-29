use metal::Device;

pub fn get_gpu_name() -> String {
  let devices = Device::all();
  let device = devices[0].name();

  device.to_string()
}

pub fn get_gpu_vram() -> u64 {
  let devices = Device::all();
  let device = devices[0].recommended_max_working_set_size();

  device
}