use std::sync::Arc;
use vulkano::{VulkanLibrary, instance::{Instance, InstanceCreateInfo}, device::physical::{self, PhysicalDevice}, memory::{MemoryHeap, MemoryHeapFlags}};

static mut INSTANCE: Option<Arc<Instance>> = None;

pub fn maybe_create_instance() {
  unsafe {
    if INSTANCE.is_none() {
      let lib = VulkanLibrary::new().unwrap();
      let instance = Instance::new(
        lib,
        InstanceCreateInfo::application_from_cargo_toml()
      );

      INSTANCE = Some(instance.unwrap());
    }
  }
}

unsafe fn get_device() -> Result<Arc<PhysicalDevice>, Box<dyn std::error::Error>> {
  if INSTANCE.is_none() {
    maybe_create_instance();
  }

  let instance = INSTANCE.as_ref().unwrap();
  let physical_devices = instance.enumerate_physical_devices();
  let physical_devices = match physical_devices {
    Ok(physical_devices) => physical_devices,
    Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to enumerate physical devices")))
  };

  if physical_devices.len() == 0 {
    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No physical devices found")));
  }
  
  let physical_devices: Vec<Arc<PhysicalDevice>> = physical_devices.collect();
  let physical_device = physical_devices[0].clone();

  Ok(physical_device)
}

pub unsafe fn get_gpu_name() -> String {
  let device = match get_device() {
    Ok(device) => device,
    Err(_) => return String::from("N/A")
  };

  device.properties().device_name.clone()
}

pub unsafe fn get_gpu_vram() -> u64 {
  let device = match get_device() {
    Ok(device) => device,
    Err(_) => return 0
  };
  
  let heap = match device.memory_properties().memory_heaps.first() {
    Some(heap) => heap,
    None => return 0
  };

  heap.size
}
