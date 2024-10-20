use std::io::Write;
use std::time::Duration;

use clap::Parser;

use crate::command::{Command as KbdCmd, *};
use crate::keyboard::Keyboard;

type CLIResult = Result<(), Box<dyn std::error::Error>>;

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ModeArgs {
  mode: OnboardMode,
  #[arg(short, long, default_value = "none")]
  dir: WaveDirection,
  #[arg(short, long, default_value = "keyboard")]
  zone: Zone,
  #[arg(short, long, default_value = "1")]
  speed: f32,
  #[arg(short, long, default_value = "100")]
  brightness: u8,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct KeyTestArgs {
  #[arg(short, long, default_value = "4")]
  start_value: u8,
  #[arg(short, long)]
  no_wait: bool,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct KeyArgs {
  keys: Vec<crate::keys::KeyCode>,
  #[arg(short, long, default_value = "100")]
  brightness: u8,
}

#[derive(Parser, Debug)]
pub enum Command {
  /// Invoke one of the built-in modes
  SetMode(ModeArgs),
  /// Set a list of keys to be on or off
  SetKeys(KeyArgs),
  /// [dev] run the key-code test helper
  KeyTest(KeyTestArgs),
}

pub struct CLI<'a> {
  cmd: Command,
  kbd: &'a Keyboard<'a>,
}

impl<'a> CLI<'a> {
  pub fn new(cmd: Command, kbd: &'a Keyboard) -> Self {
    Self { cmd, kbd }
  }

  pub fn invoke(self) -> Result<(), Box<dyn std::error::Error>> {
    match self.cmd {
      Command::SetMode(ref args) => self.set_mode(&args),
      Command::SetKeys(ref args) => self.set_keys(&args),
      Command::KeyTest(ref args) => self.key_test(&args),
    }
  }

  fn set_mode(&self, args: &ModeArgs) -> CLIResult {
    let mode = KbdCmd::OnboardMode {
      dir: args.dir,
      speed: (args.speed.clamp(0.0, 10.0) * 1_000.0) as u16,
      mode: args.mode,
      zone: args.zone,
      brightness: ((args.brightness.clamp(0, 100) as f32 / 100.0) * 255.0) as u8,
    };
    self.kbd.exec(&mode)?;
    self.kbd.commit()?;
    Ok(())
  }

  fn set_keys(&self, args: &KeyArgs) -> CLIResult {
    let brightness = ((args.brightness.clamp(0, 100) as f32 / 100.0) * 255.0) as u8;

    args.keys.chunks(12).for_each(|slice| {
      let indexes = slice.iter().map(|v| *v as u8).collect();
      let cmd = KbdCmd::KeyColor {
        indexes,
        brightness,
      };
      self.kbd.exec(&cmd).ok();
    });
    self.kbd.commit()?;
    Ok(())
  }

  fn key_test(&self, args: &KeyTestArgs) -> CLIResult {
    let mut index = args.start_value;
    let mut buf = String::new();
    let off = KbdCmd::OnboardMode {
      mode: OnboardMode::Off,
      zone: Zone::Keyboard,
      dir: WaveDirection::None,
      speed: 0,
      brightness: 0,
    };
    loop {
      let test = KbdCmd::KeyColor {
        indexes: vec![index],
        brightness: 255,
      };
      self.kbd.exec(&off)?;
      self.kbd.exec(&test)?;
      self.kbd.commit()?;
      print!("index = {:x} => ", index);
      if !args.no_wait {
        std::io::stdout().flush().ok();
        std::io::stdin().read_line(&mut buf)?;
      } else {
        println!();
        std::thread::sleep(Duration::from_millis(500));
      }
      index += 1;
    }
  }
}
