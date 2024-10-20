use clap::Parser;
use hidapi::HidApi;
use std::error::Error;

mod cli;
/// Provides the enum values and serialization for actually driving the g610 keyboard LEDs.
///
/// Based off of <https://openrgb-wiki.readthedocs.io/en/latest/Logitech-Keyboards/>
/// Some changes to values below based on missing/incorrect data in the reference.
mod command;
/// Provides a basic wrapper around the HidApi for writing commands to the keyboard.
mod keyboard;
mod keys;

use cli::{Command, CLI};
use keyboard::Keyboard;

fn main() -> Result<(), Box<dyn Error>> {
  let mut api = HidApi::new_without_enumerate()?;
  api.add_devices(0x46d, 0xc338)?;

  let info = Keyboard::locate(&api).ok_or_else(|| "No matching device found")?;
  let kbd = Keyboard::new(info, &api)?;
  println!("Located {}", kbd.description());

  let cmd = Command::parse();
  let cli = CLI::new(cmd, &kbd);

  cli.invoke()
}
