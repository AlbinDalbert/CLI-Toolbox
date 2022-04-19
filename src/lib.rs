#![allow(dead_code)]
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

// ----------------- Help ---------------------------------
// -- The help struct is used for making a costum help command for the progran.
// -- The help is constructed of a Style and the content. 
// TODO The color of the text will be costumicable, by default it is cyan.
pub struct Help {
    style: Style,
    content: str,
}

impl Help {

    pub fn new(&mut self){
        self.style = Style::new().cyan();
    }


    pub fn help(&self) {
        println!("{}", self.style.apply_to(&self.content));
    }   
    
}


//--------------------- System -----------------------------------------
// -- The system is the main print that is intended to be used as the "hub"
// -- THe main menu and navigating between the programs. 
// -- All programs are in a list within the system struct.

pub struct System {
    name: String,
    style: Style,
    sleep: u64,
    programs: Vec<Program>, 
}

// TODO add optional color for the system, green is default
pub fn new_system(name:String) -> System{

    System {
        name, 
        style: Style::new().green(), 
        sleep: 0, 
        programs: Vec::new()
    }
}

impl System {
       
    pub fn program_menu(&self) -> i32{
        let mut i: i16 = 0;
        for p in &self.programs {
            println!("{}t/{}",i ,p.name);
            i+=1;
        }

        input().parse::<i32>().unwrap_or(-1)
    }

    pub fn sys(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program(&mut self, name: String, sleep: Option<u64>){
        
        self.programs.push(new_program(name, sleep.unwrap_or(self.sleep)));
        
    }

}

// TODO add optional color, default is green, or inherited by it's parent 'System' if it has one.
pub fn new_program(name: String, sleep: u64) -> Program {
    Program { name, style: Style::new().green(), sleep }
}

// --------------------- Program (private) -------------------------
// ----- programs are handled through the 'System' ------

pub struct Program {
    name: String,
    style: Style,
    sleep: u64,
}

// TODO add function for changing color
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
// TODO The color will be costumisable, and by default it will be the Systems, 
// and it can be changed to be a given program

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
