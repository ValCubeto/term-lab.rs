//! ## Important
//! *Please check if you're printing your text on a terminal, otherwise it probably won't work.*
//! This module is only for terminals that support ANSI escape codes.

use std::fmt;
use crate::def_strings;

def_strings! {
  RESET = "0";
  BOLD = "1";
  ITALIC = "3";
  UNDERLINE = "4";
  STRIKETHROUGH = "9";
  BOLD_END = "22";
  ITALIC_END = "23";
  UNDERLINE_END = "24";
  STRIKETHROUGH_END = "29";

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
  FOREGROUND_END = "39";

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

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

macro_rules! style_fn {
  ($($(#[$tag:meta])? fn $fn_name:ident(): $start:ident, $end:ident;)*) => {
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
  #[inline(always)]
  /// Resets the text to its default style.
  fn reset() -> String {
    format!("\x1b[{}m", RESET)
  }

  style_fn! {
    #[doc = "Makes the text bold."]
    fn bold(): BOLD, BOLD_END;
    #[doc = "Makes the text cursive."]
    fn italics(): ITALIC, ITALIC_END;
    #[doc = "Adds a line under the text."]
    fn underline(): UNDERLINE, UNDERLINE_END;
    #[doc = "Adds a line through the text."]
    fn strikethrough(): STRIKETHROUGH, STRIKETHROUGH_END;

    #[doc = "Makes the text black."]
    fn black(): BLACK, FOREGROUND_END;
    #[doc = "Makes the text red."]
    fn red(): RED, FOREGROUND_END;
    #[doc = "Makes the text green."]
    fn green(): GREEN, FOREGROUND_END;
    #[doc = "Makes the text yellow."]
    fn yellow(): YELLOW, FOREGROUND_END;
    #[doc = "Makes the text blue."]
    fn blue(): BLUE, FOREGROUND_END;
    #[doc = "Makes the text magenta."]
    fn magenta(): MAGENTA, FOREGROUND_END;
    #[doc = "Makes the text cyan."]
    fn cyan(): CYAN, FOREGROUND_END;
    #[doc = "Makes the text white."]
    fn white(): WHITE, FOREGROUND_END;

    #[doc = "Makes the text bright black."]
    fn bright_black(): BRIGHT_BLACK, FOREGROUND_END;
    #[doc = "Makes the text bright red."]
    fn bright_red(): BRIGHT_RED, FOREGROUND_END;
    #[doc = "Makes the text bright green."]
    fn bright_green(): BRIGHT_GREEN, FOREGROUND_END;
    #[doc = "Makes the text bright yellow."]
    fn bright_yellow(): BRIGHT_YELLOW, FOREGROUND_END;
    #[doc = "Makes the text bright blue."]
    fn bright_blue(): BRIGHT_BLUE, FOREGROUND_END;
    #[doc = "Makes the text bright magenta."]
    fn bright_magenta(): BRIGHT_MAGENTA, FOREGROUND_END;
    #[doc = "Makes the text bright cyan."]
    fn bright_cyan(): BRIGHT_CYAN, FOREGROUND_END;
    #[doc = "Makes the text bright white."]
    fn bright_white(): BRIGHT_WHITE, FOREGROUND_END;

    #[doc = "Makes the background black."]
    fn black_bg(): BLACK_BG, END_BG;
    #[doc = "Makes the background red."]
    fn red_bg(): RED_BG, END_BG;
    #[doc = "Makes the background green."]
    fn green_bg(): GREEN_BG, END_BG;
    #[doc = "Makes the background yellow."]
    fn yellow_bg(): YELLOW_BG, END_BG;
    #[doc = "Makes the background blue."]
    fn blue_bg(): BLUE_BG, END_BG;
    #[doc = "Makes the background magenta."]
    fn magenta_bg(): MAGENTA_BG, END_BG;
    #[doc = "Makes the background cyan."]
    fn cyan_bg(): CYAN_BG, END_BG;
    #[doc = "Makes the background white."]
    fn white_bg(): WHITE_BG, END_BG;

    #[doc = "Makes the background bright black."]
    fn bright_black_bg(): BRIGHT_BLACK_BG, END_BG;
    #[doc = "Makes the background bright red."]
    fn bright_red_bg(): BRIGHT_RED_BG, END_BG;
    #[doc = "Makes the background bright green."]
    fn bright_green_bg(): BRIGHT_GREEN_BG, END_BG;
    #[doc = "Makes the background bright yellow."]
    fn bright_yellow_bg(): BRIGHT_YELLOW_BG, END_BG;
    #[doc = "Makes the background bright blue."]
    fn bright_blue_bg(): BRIGHT_BLUE_BG, END_BG;
    #[doc = "Makes the background bright magenta."]
    fn bright_magenta_bg(): BRIGHT_MAGENTA_BG, END_BG;
    #[doc = "Makes the background bright cyan."]
    fn bright_cyan_bg(): BRIGHT_CYAN_BG, END_BG;
    #[doc = "Makes the background bright white."]
    fn bright_white_bg(): BRIGHT_WHITE_BG, END_BG;
    
    #[doc = "Makes the text orange."]
    fn orange(): ORANGE, FOREGROUND_END;
    #[doc = "Makes the background orange."]
    fn orange_bg(): ORANGE_BG, END_BG;
    #[doc = "Makes the text blueberry."]
    fn blueberry(): BLUEBERRY, FOREGROUND_END;
    #[doc = "Makes the background blueberry."]
    fn blueberry_bg(): BLUEBERRY_BG, END_BG;
    #[doc = "Makes the text pink."]
    fn pink(): PINK, FOREGROUND_END;
    #[doc = "Makes the background pink."]
    fn pink_bg(): PINK_BG, END_BG;
  }

  #[inline(always)]
  /// Applies a foreground color to the string.
  fn rgb(&self, Rgb(red, green, blue): Rgb) -> String {
    format!(
      "\x1b[38;2;{};{};{}m{}\x1b[{}m",
      red, green, blue,
      self,
      FOREGROUND_END
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
  ($($(#[$tag:meta])? fn $fn_name:ident(): $start:ident, $end:ident;)*) => {
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
  lazy_style_fn! {
    #[doc = "Makes the text bold."]
    fn bold(): BOLD, BOLD_END;
    #[doc = "Makes the text cursive."]
    fn italics(): ITALIC, ITALIC_END;
    #[doc = "Adds a line under the text."]
    fn underline(): UNDERLINE, UNDERLINE_END;
    #[doc = "Adds a line through the text."]
    fn strikethrough(): STRIKETHROUGH, STRIKETHROUGH_END;

    #[doc = "Makes the text black."]
    fn black(): BLACK, FOREGROUND_END;
    #[doc = "Makes the text red."]
    fn red(): RED, FOREGROUND_END;
    #[doc = "Makes the text green."]
    fn green(): GREEN, FOREGROUND_END;
    #[doc = "Makes the text yellow."]
    fn yellow(): YELLOW, FOREGROUND_END;
    #[doc = "Makes the text blue."]
    fn blue(): BLUE, FOREGROUND_END;
    #[doc = "Makes the text magenta."]
    fn magenta(): MAGENTA, FOREGROUND_END;
    #[doc = "Makes the text cyan."]
    fn cyan(): CYAN, FOREGROUND_END;
    #[doc = "Makes the text white."]
    fn white(): WHITE, FOREGROUND_END;

    #[doc = "Makes the text bright black."]
    fn bright_black(): BRIGHT_BLACK, FOREGROUND_END;
    #[doc = "Makes the text bright red."]
    fn bright_red(): BRIGHT_RED, FOREGROUND_END;
    #[doc = "Makes the text bright green."]
    fn bright_green(): BRIGHT_GREEN, FOREGROUND_END;
    #[doc = "Makes the text bright yellow."]
    fn bright_yellow(): BRIGHT_YELLOW, FOREGROUND_END;
    #[doc = "Makes the text bright blue."]
    fn bright_blue(): BRIGHT_BLUE, FOREGROUND_END;
    #[doc = "Makes the text bright magenta."]
    fn bright_magenta(): BRIGHT_MAGENTA, FOREGROUND_END;
    #[doc = "Makes the text bright cyan."]
    fn bright_cyan(): BRIGHT_CYAN, FOREGROUND_END;
    #[doc = "Makes the text bright white."]
    fn bright_white(): BRIGHT_WHITE, FOREGROUND_END;

    #[doc = "Makes the background black."]
    fn black_bg(): BLACK_BG, END_BG;
    #[doc = "Makes the background red."]
    fn red_bg(): RED_BG, END_BG;
    #[doc = "Makes the background green."]
    fn green_bg(): GREEN_BG, END_BG;
    #[doc = "Makes the background yellow."]
    fn yellow_bg(): YELLOW_BG, END_BG;
    #[doc = "Makes the background blue."]
    fn blue_bg(): BLUE_BG, END_BG;
    #[doc = "Makes the background magenta."]
    fn magenta_bg(): MAGENTA_BG, END_BG;
    #[doc = "Makes the background cyan."]
    fn cyan_bg(): CYAN_BG, END_BG;
    #[doc = "Makes the background white."]
    fn white_bg(): WHITE_BG, END_BG;

    #[doc = "Makes the background bright black."]
    fn bright_black_bg(): BRIGHT_BLACK_BG, END_BG;
    #[doc = "Makes the background bright red."]
    fn bright_red_bg(): BRIGHT_RED_BG, END_BG;
    #[doc = "Makes the background bright green."]
    fn bright_green_bg(): BRIGHT_GREEN_BG, END_BG;
    #[doc = "Makes the background bright yellow."]
    fn bright_yellow_bg(): BRIGHT_YELLOW_BG, END_BG;
    #[doc = "Makes the background bright blue."]
    fn bright_blue_bg(): BRIGHT_BLUE_BG, END_BG;
    #[doc = "Makes the background bright magenta."]
    fn bright_magenta_bg(): BRIGHT_MAGENTA_BG, END_BG;
    #[doc = "Makes the background bright cyan."]
    fn bright_cyan_bg(): BRIGHT_CYAN_BG, END_BG;
    #[doc = "Makes the background bright white."]
    fn bright_white_bg(): BRIGHT_WHITE_BG, END_BG;

    #[doc = "Makes the text orange."]
    fn orange(): ORANGE, FOREGROUND_END;
    #[doc = "Makes the background orange."]
    fn orange_bg(): ORANGE_BG, END_BG;
    #[doc = "Makes the text blueberry."]
    fn blueberry(): BLUEBERRY, FOREGROUND_END;
    #[doc = "Makes the background blueberry."]
    fn blueberry_bg(): BLUEBERRY_BG, END_BG;
    #[doc = "Makes the text pink."]
    fn pink(): PINK, FOREGROUND_END;
    #[doc = "Makes the background pink."]
    fn pink_bg(): PINK_BG, END_BG;
  }

  pub fn rgb(mut self, Rgb(red, green, blue): Rgb) -> Self {
    self.start.push(format!("38;2;{};{};{}", red, green, blue).into_boxed_str());
    self.end.push(FOREGROUND_END.into());
    self
  }
  pub fn rgb_bg(mut self, Rgb(red, green, blue): Rgb) -> Self {
    self.start.push(format!("48;2;{};{};{}", red, green, blue).into_boxed_str());
    self.end.push(END_BG.into());
    self
  }
}
