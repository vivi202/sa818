use std::{fmt::Write, io::Read};

pub struct VolumeConfig {
  value: u8,
}

impl VolumeConfig {
  fn new(value: u8) -> Self {
    return VolumeConfig { value };
  }
}

impl VolumeConfig {
  pub fn write_config<T: Read + Write>(&self, io: &mut T) -> Result<String, String> {
    return Err("todo".to_string());
  }
}
