use clap::Parser;

use crate::command::{Command as KbdCmd, *};
use crate::keyboard::Keyboard;

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
}

#[derive(Parser, Debug)]
pub enum Command {
  SetMode(ModeArgs),
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
      Command::SetMode(m) => {
        let mode = KbdCmd::OnboardMode {
          dir: m.dir,
          speed: (m.speed * 1_000.0) as u16,
          mode: m.mode,
          zone: m.zone,
        };
        self.kbd.exec(&mode)?;
        self.kbd.exec(&KbdCmd::Commit)?;
      }
    }
    Ok(())
  }
}
