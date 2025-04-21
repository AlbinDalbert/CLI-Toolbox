use console::Style;
use crate::{TermColor, set_color};
use crate::Program;

pub struct ProgramBuilder {
    name: String,
    run_func: Option<Box<dyn Fn()>>,
    color: TermColor,
    sleep: u64,
    silent: bool,
}

impl ProgramBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            run_func: None,
            color: TermColor::Green,
            sleep: 100,
            silent: false,
        }
    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.run_func = Some(Box::new(f));
        self
    }

    pub fn shell_command(mut self, cmd: impl Into<String>) -> Self {
        let command = cmd.into();
        self.run_func = Some(Box::new(move || {
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
                .expect("Failed to run command");
            println!("Command exited with: {}", status);
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

    pub fn build(self) -> Program {
        Program {
            name: self.name,
            run_func: self.run_func.expect("No action set for Program"),
            style: set_color(Style::new(), self.color),
            sleep: self.sleep,
            silent: self.silent,
        }
    }
}