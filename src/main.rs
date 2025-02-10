#![allow(unused)]

use std::io::{ stdout, stderr, stdin };
use term_lab::info::TermInfo;
use term_lab::styles::{ Stylize, Rgb };

macro_rules! debug {
  ($e:expr) => {
    println!("{} = {:?}", stringify!($e).bold(), $e);
  };
}

// I'm using main because tests deactivate stdout
fn main() {
  let cmd = std::env::args().nth(1).unwrap_or("default".to_owned());
  match cmd.as_str() {
    "default" => default(),
    "demo" => demo(),
    _ => eprintln!("Oops")
  }
}

fn default() {
  debug!(stdout().size());
  debug!(stderr().size());
  debug!(stdin().size());
  println!(
    "{}, {}, {}",
    "Bold".bold(),
    "faint".faint(),
    "italics".italics(),
  );
  println!(
    "{}, {}",
    "Slow blink".slow_blink(),
    "fast blink".fast_blink()
  );
  println!(
    "{}, ({})",
    "Inverted".inverted(),
    "invisible".invisible()
  );
  println!(
    "{}, {}, {}, {}",
    "Underline".underlined(),
    "double underline".double_underlined(),
    "overline".overlined(),
    "striked".striked()
  );
  println!(
    "{}, {}, {}, {}, {}, {}, {}, {}",
    "Black".black(),
    "red".red(),
    "green".green(),
    "yellow".yellow(),
    "blue".blue(),
    "magenta".magenta(),
    "cyan".cyan(),
    "white".white()
  );
  println!(
    "{}, {}, {}, {}, {}, {}, {}, {}",
    "Black".bright_black(),
    "red".bright_red(),
    "green".bright_green(),
    "yellow".bright_yellow(),
    "blue".bright_blue(),
    "magenta".bright_magenta(),
    "cyan".bright_cyan(),
    "white".bright_white()
  );
  println!(
    "{}, {}, {}, {}, {}, {}, {}, {}",
    "Black".black_bg(),
    "red".red_bg(),
    "green".green_bg(),
    "yellow".yellow_bg(),
    "blue".blue_bg(),
    "magenta".magenta_bg(),
    "cyan".cyan_bg(),
    "white".white_bg()
  );
  println!(
    "{}, {}, {}, {}, {}, {}, {}, {}",
    "Black".bright_black_bg(),
    "red".bright_red_bg(),
    "green".bright_green_bg(),
    "yellow".bright_yellow_bg(),
    "blue".bright_blue_bg(),
    "magenta".bright_magenta_bg(),
    "cyan".bright_cyan_bg(),
    "white".bright_white_bg()
  );
  println!(
    "{}, {}, {}",
    "Orange".orange(),
    "blueberry".blueberry(),
    "pink".pink()
  );
  println!(
    "{}, {}, {}",
    "Orange".orange_bg(),
    "blueberry".blueberry_bg(),
    "pink".pink_bg()
  );
  println!(
    "{}, {}, {}, {}",
    "Deprecated".deprecated(),
    "error".error(),
    "warning".warning(),
    "success".success()
  );
  let rgb = Rgb(110, 255, 175);
  println!(
    "{}, {}",
    "RGB foreground".rgb(rgb),
    "RGB background".rgb_bg(rgb)
  );
  let text = "Bold + italics".style()
    .bold()
    .italics()
    .build();
  println!("{} ({:?})", text, text);
  let text = "A bunch of styles"
    .style()
    .rgb(Rgb(0xff, 0xff, 0x50))
    .rgb_bg(Rgb(0x50, 0x50, 0xff)) 
    .bold()
    .faint()
    .italics()
    .double_underlined()
    .overlined()
    .striked()
    .slow_blink()
    .inverted()
    .build();
  println!("{} ({:?})", text, text);
}

fn demo() {
  // Use text styles.
  println!(
    "{}, {}, {}, {}",
    "Bold".bold(),
    "faint".faint(),
    "italics".italics(),
    "underline".underlined()
  );

  // Use your terminal's color palette.
  println!("This is {}.", "red".red());
  println!("This is {}.", "green".green());
  println!("This is {}.", "bright blue".bright_blue());
  println!("This is {}.", "orange background".orange_bg());

  // Use a custom color palette.
  println!("This is {}.", "a custom text color".rgb(Rgb(80, 200, 200)));
  println!("This is {}.", "a custom background".rgb_bg(Rgb(80, 200, 200)));

  // Use a style builder to print less characters and improve performance.
  println!("{}.", "My composed style".style().blue().yellow_bg().italics().underlined().build());

  // Some predefined styles.
  eprintln!("Deprecated: {}", "node.exe".deprecated());
  eprintln!("{}: please insert a valid input.", "Error".error());
  eprintln!("{}: the following files will be deleted.", "Warning".warning());
  eprintln!("{}: 10 files deleted.", "Success".success());

  // Get the size of the terminal
  debug!(stdout().size().ok_or("stdout is not a TTY!"));
  debug!(stderr().size().ok_or("stderr is not a TTY!"));
  debug!(stdin().size().ok_or("stdin is not a TTY!"));
}
