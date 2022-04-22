#![allow(dead_code)]
// FIXME seperated the code into modules in different files.
// have not checked on analised the visability between them yet.
pub mod input;
pub mod help;
pub mod system;
pub mod program;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

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


// ----------------- Help ---------------------------------
/// The help struct is used for making a costum help command for the progran.
/// The help is constructed of a Style and the content. 
/// The color of the text is customizable from the enum TermColor, default is cyan.
pub struct Help {
    style: Style,
    content: String,
}

pub fn new_help(color: Option<TermColor>) -> Help{
    
    let mut help = Help {
        style: Style::new(),
        content: "Default help content".to_string(),
    };

    help.style = set_color(help.style, color.unwrap_or(TermColor::Cyan));
    help
}

impl Help {

    pub fn help(&self) {
        println!("{}", self.style.apply_to("/t---/t---Help---/t---/n"));
        println!("{}", self.style.apply_to(&self.content));
    }   
    
}


//--------------------- System -----------------------------------------
// -- The system is the main print that is intended to be used as the "hub"
// -- THe main menu and navigating between the programs. 
// -- All programs are in a list within the system struct.

pub struct System {
    name: String,
    color: TermColor,
    style: Style,
    sleep: u64,
    programs: Vec<Program>, 
}


pub fn new_system(name:String, color: Option<TermColor>) -> System{

    let mut system = System {
        name,
        color: color.unwrap_or(TermColor::Green), 
        style: Style::new(), 
        sleep: 0, 
        programs: Vec::new()
    };

    system.style = set_color(system.style, color.unwrap_or(TermColor::Green));
    system
}

impl System {
       
    pub fn program_menu(&self) -> i32{
        let mut i: i16 = 0;
        for p in &self.programs {
            println!("{}t/==t/{}",i ,p.name);
            i+=1;
        }

        input().parse::<i32>().unwrap_or(-1)
    }

    pub fn sys(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program(&mut self, name: String, sleep: Option<u64>){
        
        self.programs.push(new_program(name, Some(self.color) ,sleep.unwrap_or(self.sleep)));
        
    }

}


pub fn new_program(name: String, color: Option<TermColor> ,sleep: u64) -> Program {
    let mut program = Program { 
        name, 
        style: Style::new(), 
        sleep 
    };

    program.style = set_color(program.style, color.unwrap_or(TermColor::Green));
    program
}

// --------------------- Program (private) -------------------------
// ----- programs are handled through the 'System' ------

pub struct Program {
    name: String,
    style: Style,
    sleep: u64,
}

// changing colors after initialisation removed as it's unneceseary.
impl Program {

    pub fn prog(&self,s: String) {
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, s: String){
        println!("{}", Style::new().cyan().apply_to(self.name.to_string()+" Error> "+&s.to_string()));    
    }    

}


// ------------------------- Error -----------------------------
// Standardisation for error messages. These will not be costumizable.

pub fn err_msg(s: &str){
    println!("{}", Style::new().red().apply_to("Error> ".to_string()+&s.to_string()));
}

pub fn err(){
    println!("{}", Style::new().red().apply_to("Error> Error".to_string()));
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

// ---------------------------- Quit -----------------------------------

pub fn quit() {
    println!("\n\n\t Shutting Down\n\n");
    thread::sleep(time::Duration::from_millis(200));
    exit(0);
}
