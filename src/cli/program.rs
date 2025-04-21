// --------------------- Program -------------------------------------------------
/// Path: src\program_lib.rs

use std::{thread, time};
use console::Style;
use crate::{TermColor, set_color};

use super::ProgramBuilder;

pub struct Program {
    pub name: String,
    pub run_func: Box<dyn Fn()>,
    pub(crate) style: Style,
    pub(crate) sleep: u64,
    pub(crate) silent: bool
}

// changing colors after initialization removed as it's unnecessary.
impl Program {

    pub fn builder(name: impl Into<String>) -> ProgramBuilder {
        ProgramBuilder::new(name)
    }

    // create new program
    pub fn new<F: 'static + Fn()>(name: String, run_func: F ,color: TermColor, sleep: u64, silent: bool) -> Program{
        Program {
            name,
            run_func: Box::new(run_func),
            style: set_color(Style::new(), color),
            sleep,
            silent,
        }
    }

    // create new program from shell commands
    pub fn new_from_shell_command(
        name: String,
        command: String,
        color: TermColor,
        sleep: u64,
        silent: bool,
    ) -> Program {
        Program::new(
            name,
            move || {
                let status = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .status()
                    .expect("Failed to run command");
                println!("Command exited with: {}", status);
            },
            color,
            sleep,
            silent,
        )
    }

    pub fn set_color(&mut self, color: TermColor) {
        self.style = set_color(self.style.clone(), color);
    }

    pub fn set_sleep(&mut self, sleep: u64){
        self.sleep = sleep;
    }

    pub fn set_silence(&mut self, silent: bool) {
        self.silent = silent;
    }

    pub fn get_color(&self) -> Style {
        return self.style.clone();
    }

    pub fn get_sleep(&self) -> u64 {
        return self.sleep;
    }

    pub fn get_silence(&self) -> bool {
        return self.silent;
    }

    pub fn print(&self,s: String) {
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, s: String){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+" Error> "+&s.to_string()));    
    }
    
    pub fn run(&self){
        if !self.silent {
            println!("{}", self.style.apply_to(self.name.to_string()+&" Running...".to_string()));
        }
        (self.run_func)();
    }

    pub fn err(&self, s: Option<&String>){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+
                                                    &"Error> ".to_string()+
                                                    s.unwrap_or(&"Error".to_string())));
    }
}

