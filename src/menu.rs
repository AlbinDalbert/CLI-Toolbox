pub mod menu {
    use crate::*;
    use dialoguer::*;
    // -- The main menu and navigating between the programs.

    pub fn menu(list: Vec<String>) -> i32 {
        let mut i: i16 = 0;
        for s in list {
            println!("{} -- {}",i ,s);
            i+=1;
        }

        input().parse::<i32>().unwrap_or(-1)
    }

}