pub struct History {
    time: chrono::DateTime<chrono::Local>,
    who: String,
    reason: String,
    amount: i16
}

impl History {
    pub fn new(who: String, reason: String, amount: i16) -> History {
        History {
            who,
            reason,
            time: chrono::Local::now(),
            amount
        }
    }

    fn time_unix(&self) -> i64 {
        return self.time.timestamp();
    }

    fn time_norm(&self) -> String {
        return self.time.to_rfc2822();
    }

    pub fn show_log(&self) {
        println!("User\t\t: {}", self.who);
        println!("Reason\t\t: {}", self.reason);
        println!("At time\t\t: {}", self.time_norm());
        println!("With stamp\t: {}", self.time_unix());
    }

    pub fn get_amount(&self) -> i16 {
        return self.amount
    }
}
