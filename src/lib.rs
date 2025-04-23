#![allow(dead_code)]

/// Author (Github) : AlbinDalbert
/// Date: 2022-04-24
/// License: GPL-3.0
/// path: src\lib.rs
/// Description:
///     Tools for making a simple and good looking terminal interface.
///     Design primarily for programs with multiple sub-programs.
/// 
///     The system is made of a name, a color, a style, a sleep time and a vector of programs.
///     The system can be used to print messages, add programs and run them.
///     The system can also be used to print error messages.
///     The system can be used to get input from the user.
/// 
///     The Program struct is used to store the name, a run function, a style, a sleep time and a vector of sub-programs.
///     The Program struct can be used to print messages.
///     The Program struct can be used to print error messages.
///     The Program struct can be used to run the run function.

pub mod help;
pub mod cli;
pub use cli::{System, Program, Menu};

#[macro_use] extern crate text_io;
use std::{thread, time};
use std::str;
use std::process::exit;
use console::Style;

/// enum used in the function set_style_color() to check validity of input
/// each value here should have a styles linked to it instead. This really doesn't utilize the language capabilities
#[derive(Clone, Copy)]
pub enum TermColor{
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

// -------------------------- Input --------------------------------
// A function that ask the user to input a line, the string will then be returned.
// TODO The color will be customizable, and by default it will be yellow, 
// and it can be changed to be a given program.

pub fn input(label: &str) -> String{
    let style: Style = Style::new().yellow();
    println!("{}", style.apply_to(label));
    let s: String = read!("{}\n");
    println!("{}", style.apply_to(format!("input> {}", s)));
    let s = s.replace('\r', "");
    
    if s.eq("quit"){
        quit();
        return s;
    }
    s
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