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
    silent: bool,
}

pub struct SystemBuilder {
    name: String,
    color: TermColor,
    sleep: u64,
    silent: bool,
    programs: Vec<Program>,
}

impl System {
    pub fn builder(name: impl Into<String>) -> SystemBuilder {
        SystemBuilder::new(name)
    }

    pub fn set_color(&mut self, color: TermColor) {
        self.color = color;
        self.style = set_color(self.style.clone(), color);
    }

    pub fn set_sleep(&mut self, sleep: u64){
        self.sleep = sleep;
    }

    pub fn set_silence(&mut self, silent: bool) {
        self.silent = silent;
    }
    
    pub fn show_help(&self) {
        println!("{}", self.style.apply_to(format!("=== {} Help ===", self.name)));
        for program in &self.programs {
            println!("{}", self.style.apply_to(format!("\n{}", program.name())));
            println!("  Description: {}", program.description());
            if !program.tags().is_empty() {
                println!("  Tags: {}", program.tags().join(", "));
            }
        }
    }

    pub fn menu(&mut self) -> Option<usize> {
        // Show all programs with their descriptions and tags
        for (i, program) in self.programs.iter().enumerate() {
            println!("{0: <5} {1: <30} {2}",
                self.style.apply_to(format!("{})", i)),
                self.style.apply_to(program.name()),
                program.description());
            
            if !program.tags().is_empty() {
                println!("     Tags: {}", program.tags().join(", "));
            }
        }

        // Rest of your menu logic...
        let input = input("Pick program to launch:");
        if input == "bench" {
            Self::run_bench(self);
            return None;
        } else {
            let res = input.parse::<usize>();
            let prog = match res {
                Ok(x) => x,
                Err(_) => {
                    println!("invalid input");
                    return None;
                }
            };
            
            if prog >= self.programs.len() {
                println!("invalid input");
                return None;
            } else {
                self.programs[prog].run();
                return Some(prog);
            }    
        }   
    }

    pub fn run_bench(&mut self) {
        for p in &mut self.programs {
            let prev_silent = p.get_silence();
            p.set_silence(true);
            let mute = Gag::stdout().unwrap();
            let start = SystemTime::now();
            p.run();
            let res = start.elapsed().unwrap().as_micros() as f64 / 1000.0;    
            drop(mute);
            p.set_silence(prev_silent);
            println!("{} ... bench: \t{:.2} ms", p.name(), res);   
        }
    }

    pub fn print(&mut self, s: &str){
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn add_program<F>(&mut self, name: String, run_func: F)
    where
        F: Fn() + 'static,
    {
        let program = Program::builder(name)
            .action(run_func)
            .color(self.color)
            .sleep(self.sleep)
            .silent(self.silent)
            .build();
        self.programs.push(program);
    }

    pub fn append_program(&mut self, prog: Program){
        self.programs.push(prog);
    }

    pub fn err(&self, s: Option<&String>){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+
                                                    &"Error> ".to_string()+
                                                    s.unwrap_or(&"Error".to_string())));
    }

    pub fn programs_with_tag(&self, tag: &str) -> Vec<&Program> {
        self.programs.iter()
            .filter(|p| p.has_tag(tag))
            .collect()
    }

    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.programs.iter()
            .flat_map(|p| p.get_tags())
            .collect();
        tags.sort_unstable();
        tags.dedup();
        tags
    }

    pub fn show_tagged_menu(&mut self, tag: &str) -> Option<usize> {
        let tagged_programs: Vec<_> = self.programs_with_tag(tag).into_iter().collect();
        
        println!("{}", self.style.apply_to(format!("=== Programs tagged with '{}' ===", tag)));
        for (i, program) in tagged_programs.iter().enumerate() {
            println!("{0: <5} {1: <30} {2}",
                self.style.apply_to(format!("{})", i)),
                self.style.apply_to(program.name()),
                program.description());
        }

        // Similar menu logic but using tagged_programs...
        let input = input("Pick program to launch:");
        if input == "bench" {
            Self::run_bench(self);
            return None;
        } else {
            let res = input.parse::<usize>();
            let prog = match res {
                Ok(x) => x,
                Err(_) => {
                    println!("invalid input");
                    return None;
                }
            };
            
            if prog >= tagged_programs.len() {
                println!("invalid input");
                return None;
            } else {
                tagged_programs[prog].run();
                return Some(prog);
            }    
        }   
    }
}


impl SystemBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: TermColor::Green,
            sleep: 100,
            silent: false,
            programs: Vec::new(),
        }
    }

    pub fn color(mut self, color: TermColor) -> Self {
        self.color = color;
        self
    }

    pub fn sleep(mut self, sleep: u64) -> Self {
        self.sleep = sleep;
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub fn add_program<F>(mut self, name: String, run_func: F) -> Self
    where
        F: Fn() + 'static,
    {
        let program = Program::builder(name)
            .action(run_func)
            .color(self.color)
            .sleep(self.sleep)
            .silent(self.silent)
            .build();
        self.programs.push(program);
        self
    }

    pub fn append_program(mut self, prog: Program) -> Self {
        self.programs.push(prog);
        self
    }

    pub fn build(self) -> System {
        System {
            name: self.name,
            color: self.color,
            style: set_color(Style::new(), self.color),
            sleep: self.sleep,
            silent: self.silent,
            programs: self.programs,
        }
    }
}