#[cfg(test)]
mod test {
    
    use std::io::*;
    use crate::menu::{menu, menu::menu};
    
    
    #[test]
    fn test_menu() {
        // make a Vec of 10 strings
        let mut list: Vec<String> = Vec::new();
        for i in 0..10 {
            list.push((i*3).to_string());
        }

        // call the menu function
        let i = menu(list);
        assert_eq!(i, 0);

    }


}