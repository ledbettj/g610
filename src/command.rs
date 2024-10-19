use hidapi::{HidDevice, HidError};
use std::{thread::sleep, time::Duration};

/// Specify how the keyboard should behave on startup.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StartupMode {
  Wave = 0,
  Color,
}

/// Tell the keyboard to invoke one if it's preset onboard modes.
///
/// the OpenRGB documentation says n = 3 is 'cycle' and wave is '4',
/// but in my testing the g610 doesn't seem to have a cycle mode.
/// maybe that's only for color keyboards?
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
pub enum OnboardMode {
  Off = 0,
  Static,
  Breathing,
  Wave,
}

/// Which part of the keyboard to apply a preset to.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
pub enum Zone {
  Keyboard = 0,
  Logo,
}

/// For the wave preset, which direction it should flow.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
pub enum WaveDirection {
  None = 0,
  Horizontal,
  Vertical,
  CenterOut,
  ReverseHorizontal = 6,
  CenterIn = 8,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
  Commit,
  StartupMode {
    mode: StartupMode,
  },
  OnboardMode {
    mode: OnboardMode,
    zone: Zone,
    dir: WaveDirection,
    speed: u16,
  },
  KeyColor,
}

impl Into<Vec<u8>> for &Command {
  fn into(self) -> Vec<u8> {
    self.bytes()
  }
}

impl Command {
  pub fn bytes(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(64);

    match self {
      Command::Commit => {
        buf.extend_from_slice(&[0x11, 0xff, 0x0c, 0x5a]);
        buf.resize(20, 0);
      }
      Command::StartupMode { mode } => {
        buf.extend_from_slice(&[0x11, 0xff, 0x0d, 0x5a, 0x00, 0x01, *mode as u8]);
        buf.resize(20, 0);
      }
      // open rgb doesn't have the values for the g610 here.
      Command::OnboardMode {
        mode,
        zone,
        dir,
        speed,
      } => {
        let (r, g, b) = (128, 128, 128); // todo: allow specifying
        buf.extend_from_slice(&[
          0x11,
          0xff,
          0x0d,
          0x3c,
          *zone as u8,
          *mode as u8,
          r,
          g,
          b,
          (speed >> 8) as u8,   // no idea if all this speed stuff is correct.
          (speed & 0xff) as u8, // cycle ms,
          (speed >> 8) as u8,
          (speed & 0xff) as u8, // cycle ms,
          *dir as u8,
          0x64,
          (speed & 0xff) as u8,
          0x00, // storage ?
        ]);
        buf.resize(20, 0);
      }
      Command::KeyColor => {
        buf.extend_from_slice(&[0x12, 0xff, 0x0c, 0x3d, 0x00, 0x01, 0x00, 0x0c]);
        buf.extend_from_slice(&[/* key index, r, g, b */]); // up to 12 times
        buf.resize(64, 0);
      }
    }

    buf
  }

  pub fn write(&self, dev: &HidDevice) -> Result<(), HidError> {
    let buf: Vec<u8> = self.into();
    dev.write(&buf)?;
    sleep(Duration::from_millis(10));
    Ok(())
  }
}
