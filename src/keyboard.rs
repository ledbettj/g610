use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};

use crate::command::Command;

pub struct Keyboard<'a> {
  info: &'a DeviceInfo,
  device: HidDevice,
}

impl<'a> Keyboard<'a> {
  pub fn locate(hidapi: &'a HidApi) -> Option<&DeviceInfo> {
    hidapi
      .device_list()
      .find(|dev| dev.vendor_id() == 0x046d && dev.product_id() == 0xc338)
  }

  pub fn new(info: &'a DeviceInfo, hidapi: &'a HidApi) -> Result<Self, HidError> {
    let device = info.open_device(hidapi)?;
    Ok(Self { info, device })
  }

  pub fn description(&self) -> String {
    let s = format!(
      "{}: {}",
      self
        .info
        .manufacturer_string()
        .unwrap_or("Unknown Manufacturer"),
      self.info.product_string().unwrap_or("Unknown Product"),
    );

    s.to_owned()
  }

  pub fn exec(&self, cmd: &Command) -> Result<(), HidError> {
    cmd.write(&self.device)
  }

  pub fn commit(&self) -> Result<(), HidError> {
    self.exec(&Command::Commit)
  }
}
