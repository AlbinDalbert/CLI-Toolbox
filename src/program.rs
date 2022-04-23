
// --------------------- Program -------------------------------------------------
/// Language: rust
/// Path: src\program_lib.rs



use std::{thread, time};
use console::Style;

pub struct Program {
    pub name: String,
    style: Style,
    sleep: u64,
}

// changing colors after initialisation removed as it's unneceseary.
impl Program {

    pub fn prog(&self,s: String) {
        println!("{}", self.style.apply_to(self.name.to_string()+&"> ".to_string()+&s.to_string()));
        thread::sleep(time::Duration::from_millis(self.sleep));
    }

    pub fn err_msg(&self, s: String){
        println!("{}", Style::new().cyan().apply_to(self.name.to_string()+" Error> "+&s.to_string()));    
    }    

}

//create new program
pub fn new_program(name: String, color: Option<crate::TermColor>, sleep: u64) -> Program{
    let mut program = Program {
        name,
        style: Style::new(),
        sleep,
    };

    program.style = crate::set_color(program.style, color.unwrap_or(crate::TermColor::Green));
    program
}
