use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, EnumIter, clap::ValueEnum)]
#[repr(u8)]
pub enum KeyCode {
  A = 4,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  #[value(name = "1")]
  One,
  #[value(name = "2")]
  Two,
  #[value(name = "3")]
  Three,
  #[value(name = "4")]
  Four,
  #[value(name = "5")]
  Five,
  #[value(name = "6")]
  Six,
  #[value(name = "7")]
  Seven,
  #[value(name = "8")]
  Eight,
  #[value(name = "9")]
  Nine,
  #[value(name = "0")]
  Zero,
  Enter,
  Esc,
  Backspace,
  Tab,
  Space,
  Minus,
  Equals,
  LeftSquare,
  RightSquare,
  Backslash,
  Semicolon = 0x33,
  SingleQuote,
  Backtick,
  Comma,
  Period,
  ForwardSlash,
  CapsLock,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  PrintScreen,
  ScrollLock,
  Break,
  Insert,
  Home,
  PageUp,
  Delete,
  End,
  PageDown,
  Right,
  Left,
  Down,
  Up,
  NumLock,
  KeypadSlash,
  KeypadStar,
  KeypadMinus,
  KeypadPlus,
  KeypadEnter,
  Keypad1,
  Keypad2,
  Keypad3,
  Keypad4,
  Keypad5,
  Keypad6,
  Keypad7,
  Keypad8,
  Keypad9,
  Keypad0,
  KeypadDot,
  MenuButton = 0x65,
  LeftCtrl = 0xe0,
  LeftShift,
  LeftAlt,
  LeftWin,
  RightCtrl,
  RightShift,
  RightAlt,
  RightWin,
}

impl KeyCode {
  pub fn is_alpha(&self) -> bool {
    let range = (Self::A as u8)..=(Self::Z as u8);
    range.contains(&(*self as u8))
  }
  pub fn is_numeric(&self) -> bool {
    let range = (Self::One as u8)..=(Self::Zero as u8);
    range.contains(&(*self as u8))
  }

  pub fn is_fn(&self) -> bool {
    let range = (Self::F1 as u8)..=(Self::F12 as u8);
    range.contains(&(*self as u8))
  }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, clap::ValueEnum)]
pub enum KeyGroup {
  Alpha,
  AlphaNum,
  Num,
  Fn,
}

impl KeyGroup {
  pub fn to_keys(&self) -> Vec<KeyCode> {
    match self {
      Self::Alpha => KeyCode::iter().filter(|k| k.is_alpha()).collect(),
      Self::AlphaNum => KeyCode::iter()
        .filter(|k| k.is_alpha() || k.is_numeric())
        .collect(),
      Self::Num => KeyCode::iter().filter(|k| k.is_numeric()).collect(),
      Self::Fn => KeyCode::iter().filter(|k| k.is_fn()).collect(),
    }
  }
}
