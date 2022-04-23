///----------------- Help ---------------------------------
/// The help struct is used for making a costum help command for the progran.
/// The help is constructed of a Style and the content. 
/// The color of the text is customizable from the enum TermColor, default is cyan.

pub mod help {
    use console::Style;

    use crate::TermColor;
    pub struct Help {
        style: Style,
        content: String,
    }

    pub fn new_help(color: Option<TermColor>) -> Help{
        
        let mut help = Help {
            style: Style::new(),
            content: "Default help content".to_string(),
        };

        help.style = crate::set_color(help.style, color.unwrap_or(TermColor::Cyan));
        help
    }

    impl Help {

        pub fn help(&self) {
            println!("{}", self.style.apply_to("/t---/t---Help---/t---/n"));
            println!("{}", self.style.apply_to(&self.content));
        }   
        
    }
}