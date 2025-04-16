// --------------------- Program -------------------------------------------------
/// Path: src\program_lib.rs

use std::{thread, time};
use console::Style;

use crate::{TermColor, set_color};

#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub run_func: fn(),
    style: Style,
    sleep: u64,
}

// changing colors after initialization removed as it's unnecessary.
impl Program {

    //create new program
    pub fn new(name: String, run_func: fn() ,color: TermColor, sleep: u64) -> Program{
        Program {
            name,
            run_func,
            style: set_color(Style::new(), color),
            sleep,
        }
    }

    pub fn set_color(&mut self, color: TermColor) {
        self.style = set_color(self.style.clone(), color);
    }

    pub fn set_sleep(&mut self, sleep: u64){
        self.sleep = sleep;
    }

    pub fn print(&self,s: String) {
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, s: String){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+" Error> "+&s.to_string()));    
    }
    
    pub fn run(&self){
        println!("{}", self.style.apply_to(self.name.to_string()+&" Running...".to_string()));
        (self.run_func)();
    }

    pub fn err(&self, s: Option<&String>){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+
                                                    &"Error> ".to_string()+
                                                    s.unwrap_or(&"Error".to_string())));
    }
}

