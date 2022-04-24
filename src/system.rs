//--------------------- System --------------------------------------------------
/// path src\system.rs
/// The system is the main print that is intended to be used as the "hub"
/// The main menu and navigating between the programs. 
/// All programs are in a list within the system struct.

use std::{thread, time};
use std::str;
use console::Style;

use crate::*;
use crate::program::*;

pub struct System {
    name: String,
    color: TermColor,
    style: Style,
    sleep: u64,
    programs: Vec<Program>,
}

impl System {

    pub fn new(name:String, color: Option<TermColor>, sleep: Option<u64>) -> System{

        let mut system = System {
            name,
            color: color.unwrap_or(TermColor::Green), 
            style: Style::new(), 
            sleep: sleep.unwrap_or(100), 
            programs: Vec::new()
        };
    
        system.style = crate::set_color(system.style, color.unwrap_or(TermColor::Green));
        system
    }
    
    pub fn menu(&self) -> usize{
        let mut i: usize = 0;
        for p in &self.programs {

            println!("{0: <10} {1: <100}",
            self.style.apply_to(i.to_string()+")"),
            self.style.apply_to(p.name.clone()));

            i+=1;
        }
        i
        //let inp = input("Pick program to launch:").parse::<usize>().unwrap();
        // self.programs[inp].clone()
    }

    pub fn print(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program(&mut self, name: String, run_func: fn(), sleep: Option<u64>){
        
        self.programs.push(Program::new(name, run_func ,Some(self.color) ,sleep.unwrap_or(self.sleep)));
        
    }

    pub fn run_program(&mut self, index: usize){
        let p = self.programs[index].clone();
        p.run();
    }

    pub fn err(&self, s: Option<&String>){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+
                                                    &"Error> ".to_string()+
                                                    s.unwrap_or(&"Error".to_string())));
    }
}