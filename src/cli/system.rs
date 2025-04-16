//--------------------- System --------------------------------------------------
/// path src\system.rs
/// The system is the main print that is intended to be used as the "hub"
/// The main menu and navigating between the programs. 
/// All programs are in a list within the system struct.

use std::{thread, time};
use std::str;
use console::Style;
use std::time::SystemTime;
use gag::Gag;

use crate::*;
use crate::cli::program::*;

pub struct System {
    name: String,
    color: TermColor, // Color doesn't set the color of the system, but it's used for programs to inherit the systems color.
    style: Style,
    sleep: u64,
    programs: Vec<Program>,
}

impl System {

    pub fn new(name:String) -> System{
        System {
            name,
            color: TermColor::Green, 
            style: set_color(Style::new(), TermColor::Green), 
            sleep: 100, 
            programs: Vec::new()
        }
    }

    pub fn set_color(&mut self, color: TermColor) {
        self.color = color;
        self.style = set_color(self.style.clone(), color);
    }

    pub fn set_sleep(&mut self, sleep: u64){
        self.sleep = sleep;
    }
    
    pub fn menu(&self) {
        let mut i: usize = 0;
        for p in &self.programs {

            println!("{0: <10} {1: <100}",
            self.style.apply_to(i.to_string()+")"),
            self.style.apply_to(p.name.clone()));

            i+=1;
        }
        
        let input = input("Pick program to launch:");
        if input == "bench" {
            Self::run_bench(self);
        } else {
            let prog = input.parse::<usize>().unwrap();
            if prog > i {
                println!("invalid input");
            } else {
                self.programs[prog].run();
            }    
        }
    }

    pub fn run_bench(&self) {
        for p in &self.programs {

            let mute = Gag::stdout().unwrap();
            let start = SystemTime::now();
            p.run();
            let res = start.elapsed().unwrap().as_micros() as f64 / 1000.0;    
            drop(mute);
            println!("{} ... bench: \t{:.2} ms", p.name.clone(), res);   
            
        }
    }

    pub fn print(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program(&mut self, name: String, run_func: fn()){
        self.programs.push(Program::new(name, run_func , self.color, self.sleep));
    }

    pub fn append_program(&mut self, prog: Program){
        self.programs.push(prog);
    }

    pub fn err(&self, s: Option<&String>){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+
                                                    &"Error> ".to_string()+
                                                    s.unwrap_or(&"Error".to_string())));
    }
}