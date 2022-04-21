
// -------------------------- Input --------------------------------
// A function that askes the user to input a line, the string will then be returned.
// TODO The color will be costumisable, and by default it will be yellow, 
// TODO make a struct for input so it can be handled accordingly.
// and it can be changed to be a given program.
pub mod input {
    
    use std::str;
    use console::Style;
    use crate::quit;

    pub fn input() -> String{
        let style: Style = Style::new().yellow();
        println!("{}", style.apply_to("listening> "));
        let s: String = read!("{}\n");
        println!("{}", style.apply_to("input> ".to_owned()+&s.to_string()));
        
        if s.eq("quit"){
            quit();
            return s;
        }
        s
    }
}