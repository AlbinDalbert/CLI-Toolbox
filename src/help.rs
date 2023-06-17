///----------------- Help ---------------------------------
/// path: src\help.rs
/// The help struct is used for making a custom help command for the program.
/// The help is constructed of a Style and the content. 
/// The color of the text is customizable from the enum TermColor, default is cyan.

pub mod help {
    use console::Style;

    use crate::TermColor;

    pub struct Help {
        style: Style,
        content: String,
    }
    // TODO: Make help have a default content will all systems, programs and commands.
    impl Help {

        pub fn new(color: Option<TermColor>) -> Help{
        
            let mut help = Help {
                style: Style::new(),
                content: "Default help content".to_string(),
            };
    
            help.style = crate::set_color(help.style, color.unwrap_or(TermColor::Cyan));
            help
        }

        pub fn help(&self) {
            println!("{}", self.style.apply_to("/t---/t---Help---/t---/n"));
            println!("{}", self.style.apply_to(&self.content));
        }   
    }
}