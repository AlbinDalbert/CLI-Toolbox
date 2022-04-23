#![allow(dead_code)]

/// Author (Github) : AlbinDalbert

pub mod help;
pub mod system;
pub mod program;
// pub mod menu;

#[macro_use] extern crate text_io;
use std::{thread, time};
use std::str;
use std::process::exit;
use console::Style;

/// enum used in the function set_style_color() to check validity of input
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
// A function that askes the user to input a line, the string will then be returned.
// TODO The color will be costumisable, and by default it will be yellow, 
// TODO make a struct for input so it can be handled accordingly.
// and it can be changed to be a given program.

pub fn input() -> String{
    let style: Style = Style::new().yellow();
    println!("{}", style.apply_to("listening> "));
    let s: String = read!("{}\n");
    println!("{}", style.apply_to("input> ".to_owned()+&s.to_string()));
    
    if s.eq("quit"){
        quit();
        return s;
    }
    s
}


// ------------------------- Error -----------------------------
/// Standardisation for error messages. These will not be costumizable.
pub fn err_msg(s: &str){
    println!("{}", Style::new().red().apply_to("Error> ".to_string()+&s.to_string()));
}

pub fn err(){
    println!("{}", Style::new().red().apply_to("Error> Error".to_string()));
}

// ---------------------------- Quit -----------------------------------
pub fn quit() {
    println!("\n\n\t Shutting Down\n\n");
    thread::sleep(time::Duration::from_millis(200));
    exit(0);
}
