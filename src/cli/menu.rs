use crate::input;

/// A general purpose menu for picking a function to run.
pub struct Menu {
    name: String,
    items: Vec<Item>,
}

impl Menu {
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, name: String, func: fn()) {
        self.items.push(Item::new(name, func));
    }

    pub fn run(&self) {
        let mut i: i16 = 0;
        for Item { name, func: _ } in &self.items {
            println!("{0: <10}) {1: <100}", i, name);
            i += 1;
        }

        let inp = input("What to launch:").parse::<i32>().unwrap_or(-1);
        (self.items[inp as usize].func)();
    }
}

struct Item {
    name: String,
    func: fn(),
}

impl Item {
    fn new(name: String, func: fn()) -> Item {
        Item { name, func }
    }
}