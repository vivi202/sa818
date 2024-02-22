use std::{fmt::Write, io::Read};

pub struct VolumeConfig {
  value: u8,
}

impl VolumeConfig {
  pub fn write_config<T: Read + Write>(&self, io: &mut T) -> Result<String, String> {}
}
