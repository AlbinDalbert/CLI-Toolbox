//--------------------- System --------------------------------------------------
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


pub fn new_system(name:String, color: Option<TermColor>) -> System{

    let mut system = System {
        name,
        color: color.unwrap_or(TermColor::Green), 
        style: Style::new(), 
        sleep: 0, 
        programs: Vec::new()
    };

    system.style = crate::set_color(system.style, color.unwrap_or(TermColor::Green));
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

