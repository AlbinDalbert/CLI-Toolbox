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

// First, let's define our error types
#[derive(Debug)]
pub enum CliError {
    InvalidInput(String),
    CommandFailed(String),
    ProgramNotFound(String),
    // Add more as needed
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CliError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            CliError::ProgramNotFound(msg) => write!(f, "Program not found: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

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

    pub fn get_sleep(&self) -> u64 {
        self.sleep
    }

    pub fn get_silence(&self) -> bool {
        self.silent
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


    pub fn menu_complex_filter(&mut self, filter: Option<Box<dyn Fn(&Program) -> bool>>) -> Option<usize>{
        let filtered_indices: Vec<usize> = match filter {
            Some(f) => self.programs.iter().enumerate().filter(|(_, p)| f(p)).map(|(i, _)| i).collect(),
            None => self.programs.iter().enumerate().map(|(i, _)| i).collect(),
        };

        return print_menu(self, filtered_indices)
    }

    pub fn menu_with_tags_filter(&mut self, tags: Vec<String>) -> Option<usize> {
        let filtered_indices: Vec<usize> = self.programs.iter().enumerate()
            .filter(|(_, p)| tags.iter().all(|tag| p.tags().contains(tag)))
            .map(|(i, _)| i)
            .collect();
        print_menu(self, filtered_indices);
        None
    }
    

    pub fn menu(&mut self) -> Option<usize> {
        let indices = self.programs.iter().enumerate().map(|(i, _)| i).collect();
        return print_menu(self, indices);
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

    pub fn add_program_with_inheritance<F>(&mut self, name: String, run_func: F)
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

    pub fn err(&self, s: Option<&String>) {
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

    /// Cleanly shuts down the system
    pub fn quit(&self) {
        println!("\n\n\t Shutting Down\n\n");
        thread::sleep(time::Duration::from_millis(200));
        std::process::exit(0);
    }

    pub fn input(&self, label: &str) -> String {
        println!("{}", self.style.apply_to(label));
        let s: String = read!("{}\n");
        println!("{}", self.style.apply_to(format!("{}> {}", self.name, s)));
        let s = s.replace('\r', "");
        
        if s.eq("quit") {
            self.quit();
        }
        s
    }

    pub fn input_with_validation<F>(&self, label: &str, validator: F) -> String 
    where
        F: Fn(&str) -> bool
    {
        loop {
            let input = self.input(label);
            if validator(&input) {
                return input;
            }
            self.err(Some(&"Invalid input".to_string()));
        }
    }

    pub fn input_number(&self, label: &str) -> Option<i32> {
        self.input(label).parse().ok()
    }

    pub fn run_program(&self, index: usize) -> Result<(), CliError> {
        if index >= self.programs.len() {
            return Err(CliError::ProgramNotFound(format!("Index {} out of bounds", index)));
        }
        self.programs[index].run();
        Ok(())
    }

    pub fn input_required(&self, label: &str) -> Result<String, CliError> {
        let input = self.input(label);
        if input.is_empty() {
            Err(CliError::InvalidInput("Input cannot be empty".to_string()))
        } else {
            Ok(input)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn programs(&self) -> &[Program] {
        &self.programs
    }

    pub fn display(&self) {
        println!("System Name: {}", self.name);
        println!("Color: {:?}", self.color.to_string());
        println!("Style: {:?}", self.style);
        println!("Sleep: {}", self.sleep);
        println!("Silent: {}", self.silent);
        println!("Programs:");

        for (index, program) in self.programs.iter().enumerate() {
            println!("  Program {}:", index + 1);
            println!("    Name: {}", program.get_name());
            println!("    Color: {:?}", program.get_color());
            println!("    Sleep: {}", program.get_sleep());
            println!("    Silent: {}", program.get_silence());
            println!("    Description: {}", program.get_description());
            println!("    Tags: {:?}", program.get_tags());
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

    pub fn use_defaults(self) -> Self {
        self.color(TermColor::Green)
            .sleep(100)
            .silent(false)
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

    pub fn add_program_with_inheritance<F>(mut self, name: String, run_func: F) -> Self
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

fn print_menu(sys: &mut System, indices: Vec<usize>) -> Option<usize>{
    for i in indices {
        println!("{0: <5} {1: <30} {2}",
            sys.style.apply_to(format!("{})", i)),
            sys.style.apply_to(sys.programs[i].name()),
            sys.programs[i].description());
        
        if !sys.programs[i].tags().is_empty() {
            println!("     Tags: {}",Style::new().italic().apply_to(sys.programs[i].tags().join(", ")));
        }
    }

    let input = sys.input("Pick program to launch:");
    if input == "bench" {
        sys.run_bench();
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
        
        if prog >= sys.programs.len() {
            println!("invalid input");
            return None;
        } else {
            sys.programs[prog].run();
            return Some(prog);
        }
    }
}