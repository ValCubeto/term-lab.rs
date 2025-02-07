use term_lab::styles::{ Stylize, Rgb };

// I'm using main because tests deactivate stdout
fn main() {
  println!("{}", "Bold".bold());
  println!("{}", "Italics".italics());
  println!("{}", "Underline".underline());
  println!("{}", "Strikethrough".strikethrough());
  println!();

  println!("{}", "Black".black());
  println!("{}", "Red".red());
  println!("{}", "Green".green());
  println!("{}", "Yellow".yellow());
  println!("{}", "Blue".blue());
  println!("{}", "Magenta".magenta());
  println!("{}", "Cyan".cyan());
  println!("{}", "White".white());
  println!("{}", "Bright Black".bright_black());
  println!("{}", "Bright Red".bright_red());
  println!("{}", "Bright Green".bright_green());
  println!("{}", "Bright Yellow".bright_yellow());
  println!("{}", "Bright Blue".bright_blue());
  println!("{}", "Bright Magenta".bright_magenta());
  println!("{}", "Bright Cyan".bright_cyan());
  println!("{}", "Bright White".bright_white());
  println!("{}", "Orange".orange());
  println!("{}", "Blueberry".blueberry());
  println!("{}", "Pink".pink());
  println!();

  println!("{}", "Black background".black_bg());
  println!("{}", "Red background".red_bg());
  println!("{}", "Green background".green_bg());
  println!("{}", "Yellow background".yellow_bg());
  println!("{}", "Blue background".blue_bg());
  println!("{}", "Magenta background".magenta_bg());
  println!("{}", "Cyan background".cyan_bg());
  println!("{}", "White background".white_bg());
  println!("{}", "Bright black background".bright_black_bg());
  println!("{}", "Bright red background".bright_red_bg());
  println!("{}", "Bright green background".bright_green_bg());
  println!("{}", "Bright yellow background".bright_yellow_bg());
  println!("{}", "Bright blue background".bright_blue_bg());
  println!("{}", "Bright magenta background".bright_magenta_bg());
  println!("{}", "Bright cyan background".bright_cyan_bg());
  println!("{}", "Bright white background".bright_white_bg());
  println!("{}", "Orange background".orange_bg());
  println!("{}", "Blueberry background".blueberry_bg());
  println!("{}", "Pink background".pink_bg());
  println!();

  let rgb = Rgb(110, 255, 175);
  println!("{}", "RGB".rgb(rgb));
  println!("{}", "RGB Background".rgb_bg(rgb));
  println!();

  println!("{}", "Deprecated".deprecated());
  println!("{}", "Error".error());
  println!("{}", "Warning".warning());
  println!("{}", "Success".success());
  println!();

  let text = "Bold + italics".style()
    .bold()
    .italics()
    .build();
  println!("{} ({:?})", text, text);
  let text = "test"
    // .style()
    .rgb(Rgb(0xff, 0x50, 0x50))
    .rgb_bg(Rgb(0x50, 0x50, 0xff)) 
    .bold()
    .italics()
    .underline()
    .strikethrough()
    // .build()
    ;
  println!("{} ({:?})", text, text);
}
