// --------------------- Program -------------------------------------------------
/// Path: src\program_lib.rs

use std::{thread, time};
use console::Style;
use crate::{TermColor, set_color};

pub trait CommandExecutor {
    fn execute(&self, cmd: &str) -> std::process::ExitStatus;
}

pub struct DefaultCommandExecutor;

impl CommandExecutor for DefaultCommandExecutor {
    fn execute(&self, cmd: &str) -> std::process::ExitStatus {
        #[cfg(unix)]
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .expect("Failed to run command");

        #[cfg(windows)]
        let status = std::process::Command::new("cmd")
            .arg("/C")
            .arg(cmd)
            .status()
            .expect("Failed to run command");

        status
    }
}

pub struct Program {
    name: String,
    run_func: Box<dyn Fn()>,
    style: Style,
    sleep: u64,
    silent: bool,
    description: String,
    tags: Vec<String>,
}

pub struct ProgramBuilder {
    name: String,
    run_func: Option<Box<dyn Fn()>>,
    color: TermColor,
    sleep: u64,
    silent: bool,
    description: String,
    tags: Vec<String>,
}


// changing colors after initialization removed as it's unnecessary.
impl Program {

    pub fn builder(name: impl Into<String>) -> ProgramBuilder {
        ProgramBuilder::new(name)
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

    pub fn print(&self, message: &str) {
        println!("{}", self.style.apply_to(format!("{}> {}", self.name, message)));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, message: &str) {
        println!("{}", Style::new().red().apply_to(format!("{} Error> {}", self.name, message)));    
    }
    
    pub fn run(&self) {
        if !self.silent {
            println!("{}", self.style.apply_to(format!("{} Running...", self.name)));
        }
        (self.run_func)();
    }

    pub fn err(&self, message: &str) {
        println!("{}", Style::new().red().apply_to(format!("{} Error> {}", self.name, message)));
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
    
}


impl ProgramBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            run_func: None,
            color: TermColor::Green,
            sleep: 0,
            silent: false,
            description: String::new(),
            tags: Vec::new(),
        }
    }

    pub fn use_defaults(self) -> Self {
        self.color(TermColor::Green)
            .sleep(0)
            .silent(false)
            .description("")
    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.run_func = Some(Box::new(f));
        self
    }

    pub fn shell_command(mut self, cmd: impl Into<String>) -> Self {
        let command = cmd.into();
        let executor = DefaultCommandExecutor;
        self.run_func = Some(Box::new(move || {
            let status = executor.execute(&command);
            if !status.success() {
                println!("Command failed with status: {}", status);
            }
        }));
        self
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

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn build(self) -> Program {
        Program {
            name: self.name,
            run_func: self.run_func.expect("No action set for Program"),
            style: set_color(Style::new(), self.color),
            sleep: self.sleep,
            silent: self.silent,
            description: self.description,
            tags: self.tags,
        }
    }
}
