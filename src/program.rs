// --------------------- Program -------------------------------------------------
/// Language: rust
/// Path: src\program_lib.rs

use std::{thread, time};
use console::Style;

#[derive(Clone)]
pub struct Program {
    pub name: String,
    run_func: fn(&mut Program),
    style: Style,
    sleep: u64,
}

// changing colors after initialisation removed as it's unneceseary.
impl Program {

    //create new program
    pub fn new(name: String, run_func:fn(&mut Program) ,color: Option<crate::TermColor>, sleep: u64) -> Program{
        let mut program = Program {
            name,
            run_func,
            style: Style::new(),
            sleep,
        };

        program.style = crate::set_color(program.style, color.unwrap_or(crate::TermColor::Green));
        program
    }

    pub fn prog(&self,s: String) {
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, s: String){
        println!("{}", Style::new().red().apply_to(self.name.to_string()+" Error> "+&s.to_string()));    
    }
    
    pub fn run(&self){
        (self.run_func);
    }

}

