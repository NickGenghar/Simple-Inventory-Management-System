#![allow(unused)]

use crate::History;
use crate::Item;

use std::fs::{OpenOptions, File};
use std::io::{self, Write};

pub struct Database {
    file_handler: io::Result<File>,
    items: Vec<Item>,
}

impl Database {
    pub fn create_empty(name: String) -> Option<Self> {
        {
            Ok(_) => {
                let mut name = String::from(s.trim_end());
                println!("{}", name.len());
                if(name.len() > 1) {
                    let file_handler = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&name);

                    println!("Opening handler for file: {}", &name);

                    Some(Self {
                        file_handler,
                        items: Vec::<Item>::new(),
                    })
                } else {
                    None
                }
            }
            Err(e) => {
                println!("Error! {}", e);
                None
            }
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn list_items(&self) {
        for i in &self.items {
            println!("{}", i.name);
        }
    }
}
