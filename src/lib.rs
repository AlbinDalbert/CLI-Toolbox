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
use std::{io, process::exit};
use console::Style;

// ----------------- Help ---------------------------------

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

pub struct System {
    name: String,
    style: Style,
    sleep: u64,
    programs: Vec<Program>, 
}

pub fn new_system(name:String) -> System{

    System {
        name, 
        style: Style::new().green(), 
        sleep: 0, 
        programs: Vec::new()
    }
}

impl System {
       

    pub fn sys(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program(&mut self, name: String){
        self.programs.push(new_program(name));
    }



}


fn new_program(name: String) -> Program {
    Program { name, style: Style::new().green() }
}

// --------------------- Program (private) -------------------------
// ----- programs are handled through the 'System' ------

struct Program {
    name: String,
    style: Style,
}

impl Program {
    
}


// ------------------------- Error -----------------------------

pub fn err_msg(s: &str){
    println!("{}", Style::new().cyan().apply_to("Error> ".to_string()+&s.to_string()));
}

pub fn err(){
    println!("{}", Style::new().red().apply_to("Error> Error".to_string()));
}

// -------------------------- Input --------------------------------

pub fn input() -> String{
    let style: Style = Style::new().yellow();
    println!("{}", style.apply_to("listening> "));
    let s: String = read!("{}\n");
    println!("{}", style.apply_to("input> ".to_owned()+&s.to_string()));
    s
}

// ---------------------------- Quit -----------------------------------

pub fn quit() {
    println!("\n\n\t Shutting Down\n\n");
    thread::sleep(time::Duration::from_millis(200));
    exit(0);
}
