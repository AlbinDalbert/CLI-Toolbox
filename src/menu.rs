/// A general purpose menu for picking a function to run.
/// 
struct Menu {
    name: String,
    items: Vec<(String, fn())>,
}

impl Menu {
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, name: String, func: fn()) {
        self.items.push((name, func));
    }

    pub fn run(&self) {
        let mut i: i16 = 0;
        for (name, func) in &self.items {
            println!("{0: <10}) {1: <100}", i, name);
            i += 1;
        }

        let inp = input().parse::<i32>().unwrap_or(-1);
        self.items[inp as usize].1();
    }
}
