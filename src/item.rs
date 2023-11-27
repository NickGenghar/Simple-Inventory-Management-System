use crate::History;

pub struct Item {
    pub name: String,
    pub quantity: i16,
    logs: std::vec::Vec<History>
}

impl Item {
    pub fn create(name: String) -> Self {
        let mut logs = Vec::<History>::new();

        Self {
            name,
            quantity: 0,
            logs
        }
    }

    pub fn new(name: String, quantity: i16) -> Self {
        let mut logs = std::vec::Vec::<History>::new();
        let h = History::new(String::from("Admin"), String::from("Entry creation."), quantity);
        logs.push(h);

        Self {
            name,
            quantity,
            logs
        }
    }

    pub fn add_log(&mut self, history: History) {
        self.quantity += history.get_amount();
        self.logs.push(history);
    }

    pub fn show_logs(&self) {
        println!("Showing logs for item\t: {}", self.name);
        println!("Current item quantity\t: {}", self.quantity);

        println!("\n\n");

        for i in &self.logs {
            i.show_log();
            println!("Net change\t: {}\n", i.get_amount());
        }
    }
}
