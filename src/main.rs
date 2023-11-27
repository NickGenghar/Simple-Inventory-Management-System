#![allow(unused)]
mod history;
mod item;
mod database;

use crate::history::History;
use crate::item::Item;
use crate::database::Database;

use std::io;

fn read_then_write(stdin: &io::Stdin) -> String {
    let mut s = String::new();
    stdin.read_line(&mut s);
    return String::from(s.trim_end());
}

fn main() -> io::Result<()> {
    //Open stdin
    let stdin = io::stdin();

    let db = Database::create_empty(String::from("test"));

    let who = String::from("Hello");
    let reason = String::from("World");

    let item_name = String::from("Tissue paper");

    let mut i: Item;
    //i = Item::new(item_name, 255)
    i = Item::create(item_name);
    let mut user = String::new();
    let mut val = String::new();

    i.show_logs();

    println!("Testing io read for user");
    user = read_then_write(&stdin);
    println!("Testing io read for val");
    val = read_then_write(&stdin);

    match (val.trim_end().parse::<i16>()) {
        Ok(v) => {println!("You wrote {} and {}", user, v);}
        Err(e) => {println!("You wrote {0} and {1}, but {1} is not a valid number.", user, val);}
    }

    println!("Sleeping for 2 seconds...");
    std::thread::sleep(std::time::Duration::new(2, 0));

    let h = History::new(who,reason, -12);
    i.add_log(h);
    i.show_logs();

    Ok(())
}
