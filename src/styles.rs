//! ## Important
//! *Please check if you're printing your text on a terminal, otherwise it probably won't work.*
//! This module is only for terminals that support ANSI escape codes.

use std::fmt;
use crate::def_strings;

#[cps::cps]
/// Pass a file to a macro.
/// As this 
macro_rules! pass_file {
  ($macro:ident!($source:expr)) => 
    let $($file:tt)* = cps::include!($source) in
  {
    $macro! { $($file)* }
  };
}

def_strings! {
  #[doc = "Reset all styles"]
  RESET = "0";
  #[doc = "Increase the font weight"]
  BOLD = "1";
  #[doc = "Decrease the font weight"]
  BOLD_OFF = "22";
  #[doc = "Decrease the intensity"]
  FAINT = "2";
  #[doc = "Reset the intensity"]
  FAINT_OFF = "22";
  ITALICS = "3";
  ITALICS_OFF = "23";
  UNDERLINE = "4";
  DOUBLE_UNDERLINE = "21";
  UNDERLINE_OFF = "24";
  #[doc = "Make the text blink"]
  SLOW_BLINK = "5";
  #[doc = "Make the text blink"]
  FAST_BLINK = "6";
  #[doc = "Reset the blink"]
  BLINK_OFF = "25";
  INVERSE = "7";
  INVERSE_OFF = "27";
  INVISIBLE = "8";
  INVISIBLE_OFF = "28";
  STRIKETHROUGH = "9";
  STRIKETHROUGH_OFF = "29";
  OVERLINE = "53";
  OVERLINE_OFF = "55";

  BLACK = "30";
  RED = "31";
  GREEN = "32";
  YELLOW = "33";
  BLUE = "34";
  MAGENTA = "35";
  CYAN = "36";
  WHITE = "37";
  BRIGHT_BLACK = "90";
  BRIGHT_RED = "91";
  BRIGHT_GREEN = "92";
  BRIGHT_YELLOW = "93";
  BRIGHT_BLUE = "94";
  BRIGHT_MAGENTA = "95";
  BRIGHT_CYAN = "96";
  BRIGHT_WHITE = "97";
  FOREGROUND_OFF = "39";

  BLACK_BG = "40";
  RED_BG = "41";
  GREEN_BG = "42";
  YELLOW_BG = "43";
  BLUE_BG = "44";
  MAGENTA_BG = "45";
  CYAN_BG = "46";
  WHITE_BG = "47";
  BRIGHT_BLACK_BG = "100";
  BRIGHT_RED_BG = "101";
  BRIGHT_GREEN_BG = "102";
  BRIGHT_YELLOW_BG = "103";
  BRIGHT_BLUE_BG = "104";
  BRIGHT_MAGENTA_BG = "105";
  BRIGHT_CYAN_BG = "106";
  BRIGHT_WHITE_BG = "107";
  END_BG = "49";

  ORANGE = "38;2;230;126;20";
  ORANGE_BG = "48;2;230;126;20";
  BLUEBERRY = "38;2;80;80;255";
  BLUEBERRY_BG = "48;2;80;80;255";
  PINK = "38;2;255;0;200";
  PINK_BG = "48;2;255;0;200";
}

#[inline(always)]
/// Resets the text to its default style.
pub fn reset() -> String {
  format!("\x1b[{}m", RESET)
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

macro_rules! style_fn {
  ($($(#[$tag:meta])? fn $fn_name:ident() -> ($start:ident, $end:ident);)*) => {
    $(
      $(#[$tag])?
      #[inline(always)]
      /// Displays the text based on the corresponding style predefined by the terminal.
      fn $fn_name(&self) -> String {
        format!("\x1b[{}m{}\x1b[{}m", $start, self, $end)
      }
    )*
  }
}

/// A trait that provides styling for terminals that support ANSI escape codes.
pub trait Stylize: fmt::Display {
  pass_file!(style_fn!("src/styles.in"));

  #[inline(always)]
  /// Applies a foreground color to the string.
  fn rgb(&self, Rgb(red, green, blue): Rgb) -> String {
    format!(
      "\x1b[38;2;{};{};{}m{}\x1b[{}m",
      red, green, blue,
      self,
      FOREGROUND_OFF
    )
  }
  #[inline(always)]
  /// Applies a background color to the string.
  fn rgb_bg(&self, Rgb(red, green, blue): Rgb) -> String {
    format!(
      "\x1b[48;2;{};{};{}m{}\x1b[{}m",
      red, green, blue,
      self,
      END_BG
    )
  }

  #[inline(always)]
  /// Creates a `StyleBuilder`, use it to combine multiple styles. Don't forget to call `build`!
  fn style(&self) -> StyleBuilder {
    StyleBuilder::new(self.to_string())
  }

  #[inline(always)]
  /// Applies the italics and strikethrough styles.
  fn deprecated(&self) -> String {
    format!("\x1b[3;9m{}\x1b[23;29m", self)
  }
  #[inline(always)]
  /// Makes the text bold and red.
  fn error(&self) -> String {
    self.style().red().bold().build()
  }
  #[inline(always)]
  /// Makes the text bold and yellow.
  fn warning(&self) -> String {
    self.style().yellow().bold().build()
  }
  #[inline(always)]
  /// Makes the text bold and green.
  fn success(&self) -> String {
    self.style().green().bold().build()
  }
}

impl Stylize for String {}
impl Stylize for &str {}
impl Stylize for Box<str> {}

#[derive(Debug)]
pub struct StyleBuilder {
  text: String,
  start: Vec<Box<str>>,
  end: Vec<Box<str>>,
}

macro_rules! lazy_style_fn {
  ($($(#[$tag:meta])? fn $fn_name:ident() -> ($start:ident, $end:ident);)*) => {
    $(
      $(#[$tag])?
      #[inline(always)]
      /// <br>
      /// *Remember that applying a color overwrites the previous one.*
      pub fn $fn_name(mut self) -> Self {
        self.start.push($start.into());
        self.end.push($end.into());
        self
      }
    )*
  }
}

impl StyleBuilder {
  #[inline]
  fn new(text: String) -> Self {
    Self {
      text,
      start: vec![],
      end: vec![],
    }
  }
  #[inline(always)]
  pub fn build(self) -> String {
    format!("\x1b[{}m{}\x1b[{}m", self.start.join(";"), self.text, self.end.join(";"))
  }
  pass_file!(lazy_style_fn!("src/styles.in"));

  pub fn rgb(mut self, Rgb(red, green, blue): Rgb) -> Self {
    self.start.push(format!("38;2;{};{};{}", red, green, blue).into_boxed_str());
    self.end.push(FOREGROUND_OFF.into());
    self
  }
  pub fn rgb_bg(mut self, Rgb(red, green, blue): Rgb) -> Self {
    self.start.push(format!("48;2;{};{};{}", red, green, blue).into_boxed_str());
    self.end.push(END_BG.into());
    self
  }
}
