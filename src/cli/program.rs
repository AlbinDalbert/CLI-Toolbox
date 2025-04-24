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

pub struct ShellCommand {
    base_command: String,
    args: Vec<String>,
    validate: bool,
}

#[derive(Debug)]
pub enum ShellCommandError {
    CommandNotFound(String),
    NotExecutable(String),
    ExecutionFailed(String),
}

impl ShellCommand {
    pub fn new(base_command: impl Into<String>) -> Self {
        Self {
            base_command: base_command.into(),
            args: Vec::new(),
            validate: true,
        }
    }

    pub fn add_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn with_validation(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    fn validate_command(&self) -> Result<(), ShellCommandError> {
        if !self.validate {
            return Ok(());
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&self.base_command) {
                if !metadata.is_file() {
                    return Err(ShellCommandError::NotExecutable(format!("{} is not a file", self.base_command)));
                }
                if metadata.permissions().mode() & 0o111 == 0 {
                    return Err(ShellCommandError::NotExecutable(format!("{} is not executable", self.base_command)));
                }
            } else {
                // Check if command exists in PATH
                if let Err(_) = std::process::Command::new("which")
                    .arg(&self.base_command)
                    .output() {
                    return Err(ShellCommandError::CommandNotFound(format!("Command {} not found", self.base_command)));
                }
            }
        }

        #[cfg(windows)]
        {
            // On Windows, we'll check if the command exists in PATH
            if let Err(_) = std::process::Command::new("where")
                .arg(&self.base_command)
                .output() {
                return Err(ShellCommandError::CommandNotFound(format!("Command {} not found", self.base_command)));
            }
        }

        Ok(())
    }

    pub fn execute(&self) -> Result<std::process::ExitStatus, ShellCommandError> {
        self.validate_command()?;

        let mut command = std::process::Command::new(&self.base_command);
        for arg in &self.args {
            command.arg(arg);
        }
        
        command.status()
            .map_err(|e| ShellCommandError::ExecutionFailed(format!("Failed to execute command: {}", e)))
    }
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
        let command = ShellCommand::new(cmd);
        self.run_func = Some(Box::new(move || {
            match command.execute() {
                Ok(status) => {
                    if !status.success() {
                        println!("Command failed with status: {}", status);
                    }
                }
                Err(e) => println!("Command error: {:?}", e),
            }
        }));
        self
    }

    pub fn dynamic_shell_command(mut self, cmd: impl Into<String>) -> Self {
        let command = ShellCommand::new(cmd);
        self.run_func = Some(Box::new(move || {
            match command.execute() {
                Ok(status) => {
                    if !status.success() {
                        println!("Command failed with status: {}", status);
                    }
                }
                Err(e) => println!("Command error: {:?}", e),
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
