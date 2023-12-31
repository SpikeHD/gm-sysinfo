use std::sync::Arc;
use vulkano::{
  device::physical::PhysicalDevice,
  instance::{Instance, InstanceCreateInfo},
  VulkanLibrary,
};

static mut INSTANCE: Option<Arc<Instance>> = None;

pub fn maybe_create_instance() {
  unsafe {
    if INSTANCE.is_none() {
      let lib = match VulkanLibrary::new() {
        Ok(lib) => lib,
        Err(_) => return,
      };
      let instance = Instance::new(lib, InstanceCreateInfo::application_from_cargo_toml());

      INSTANCE = match instance {
        Ok(instance) => Some(instance),
        Err(_) => None,
      };
    }
  }
}

unsafe fn get_device() -> Result<Arc<PhysicalDevice>, Box<dyn std::error::Error>> {
  if INSTANCE.is_none() {
    maybe_create_instance();

    // if it is still none, then we failed to create an instance
    if INSTANCE.is_none() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to create instance",
      )));
    }
  }

  let instance = INSTANCE.as_ref().unwrap();
  let physical_devices = instance.enumerate_physical_devices();
  let physical_devices = match physical_devices {
    Ok(physical_devices) => physical_devices,
    Err(_) => {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to enumerate physical devices",
      )))
    }
  };

  if physical_devices.len() == 0 {
    return Err(Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "No physical devices found",
    )));
  }

  let physical_devices: Vec<Arc<PhysicalDevice>> = physical_devices.collect();
  let physical_device = physical_devices[0].clone();

  Ok(physical_device)
}

pub unsafe fn get_gpu_name() -> String {
  let device = match get_device() {
    Ok(device) => device,
    Err(_) => return String::from("N/A"),
  };

  device.properties().device_name.clone()
}

pub unsafe fn get_gpu_vram() -> u64 {
  let device = match get_device() {
    Ok(device) => device,
    Err(_) => return 0,
  };

  let heap = &device.memory_properties().memory_heaps;
  let heap = heap.iter().find(|heap| {
    heap
      .flags
      .contains(vulkano::memory::MemoryHeapFlags::DEVICE_LOCAL)
  });

  if heap.is_none() {
    return 0;
  }

  heap.unwrap().size
}
