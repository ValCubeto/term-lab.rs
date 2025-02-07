# term-lab.rs
A crate to easily work with terminal styles and get terminal info.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
term-lab = "1.0.0"
```
Or run this command, *located in your project's root directory*:
```bash
  cargo add term-lab
```

## Examples
```rust
use term_lab::styles::{ Stylize, Rgb };

// Use your terminal's color palette.
println!("This is {}!", "red".red());
println!("This is {}!", "green".green());
println!("This is {}!", "bright blue".bright_blue());

// Use a style builder to print less characters and improve performance.
println!("{}", "My custom style".style().green().italics().build()); 

// Some predefined styles.
eprintln!("{}", "node.exe".deprecated());
eprintln!("{}: please insert a valid input.", "Error".error());
eprintln!("{}: the following files will be deleted.", "Warning".warning());
eprintln!("{}: 10 files deleted.", "Success".success());
```

## Performance
Use the `style` method to combine multiple styles and print less characters.

## Features
None at the moment.
