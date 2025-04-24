#![allow(dead_code)]

/// Author (Github) : AlbinDalbert
/// Date: 2022-04-24
/// License: Apache-2.0
/// 
/// # CLI Toolbox
/// 
/// A library for creating simple and good looking terminal interfaces.
/// Designed primarily for programs with multiple sub-programs.
/// 
/// ## Features
/// - System management with name, color, style, and sleep time
/// - Program management with run functions and metadata
/// - Tag-based organization
/// - Cross-platform shell command execution
/// - Error handling and input validation
/// 
/// ## Example
/// ```
/// use cli_toolbox::{System, Program, TermColor};
/// 
/// let mut system = System::builder("My Tool")
///     .use_defaults()
///     .build();
/// 
/// system.add_program_with_inheritance("hello".to_string(), || println!("Hello, world!"));
/// ```

pub mod cli;
pub use cli::{System, Program};
pub use cli::program::ShellCommand;

#[macro_use] extern crate text_io;
use std::{thread, time};
use std::process::exit;
use console::Style;

/// Color options for terminal output styling
#[derive(Clone, Copy)]
pub enum TermColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn set_color(style: Style, color: TermColor) -> Style{
    match color {
        TermColor::Black => style.black(),
        TermColor::Red => style.red(),
        TermColor::Green => style.green(),
        TermColor::Yellow => style.yellow(),
        TermColor::Blue => style.blue(),
        TermColor::Magenta => style.magenta(),
        TermColor::Cyan => style.cyan(),
        TermColor::White => style.white(),
    }
}

// ------------------------- Error -----------------------------
/// Standardization for error messages. These will not be customizable.
pub fn err(s: Option<&String>){
    println!("{}", Style::new().red().apply_to("Error> ".to_string()+s.unwrap_or(&"Error".to_string())));
}

// ---------------------------- Quit -----------------------------------
pub fn quit() {
    println!("\n\n\t Shutting Down\n\n");
    thread::sleep(time::Duration::from_millis(200));
    exit(0);
}

#[cfg(test)]
mod tests;