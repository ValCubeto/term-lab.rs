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

// Use a custom color palette.
println!("This is {}.", "custom".rgb(Rgb(80, 255, 200)));
println!("This is a {}.", "custom background".rgb_bg(Rgb(80, 255, 200)));

// Use a style builder to print less characters and improve performance.
println!("{}.", "My custom style".style().green().italics().build()); 

// Some predefined styles.
eprintln!("{}", "node.exe".deprecated());
eprintln!("{}: please insert a valid input.", "Error".error());
eprintln!("{}: the following files will be deleted.", "Warning".warning());
eprintln!("{}: 10 files deleted.", "Success".success());
```

**Output**:
<span style="font-family: monospace">
> This is <span style="color: #CC0000;">red</span>! <br>
> This is <span style="color: #4E9A06;">green</span>! <br>
> This is <span style="color: #729FCF;">bright blue</span>! <br>
> This is <span style="color: rgb(80, 255, 200);">custom</span>. <br>
> This is a <span style="color: black; background-color: rgb(80, 255, 200);">custom background</span>. <br>
> <span style="font-style: italic; color: #4E9A06;">My custom style</span>. <br>
> <strike><span style="font-style: italic">node.exe</span></strike> <br>
> <b><span style="color: #CC0000;">Error</span></b>: please insert a valid input. <br>
> <b><span style="color: #C4A000;">Warning</span></b>: the following files will be deleted. <br>
> <b><span style="color: #4E9A06;">Success</span></b>: 10 files deleted. <br>
</span>

## Performance
Use the `style` method to combine multiple styles and print less characters.

## Crate \[features\]
None at the moment.
