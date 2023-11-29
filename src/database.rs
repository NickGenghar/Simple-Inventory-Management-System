#![allow(unused)]

use crate::History;
use crate::Item;

use std::fs::{OpenOptions, File};
use std::io::{self, Write};

pub struct Database {
    file_handler: File,
    items: Vec<Item>,
}

impl Database {
    /**/pub fn create_empty(s: String) -> Option<Self> {
        let mut name = String::from(s.trim_end());
        println!("{}", name.len());
        if(name.len() > 1) {
            let mut file_handler: File;

            match File::create(&name) {
                Ok(opened) => {
                    file_handler = opened;
                    println!("Opening handler for file: {}", &name);

                    Some(Self {
                        file_handler,
                        items: Vec::<Item>::new(),
                    })
                }
                Err(e) => {
                    panic!("Database already exist. If you wish to open the database instead, please use the open function.");
                }
            }

        } else {
            panic!("Error! Failed to enumerate database.")
        }
    }/** */

    pub fn open_existing(s: String) -> Option<Self> {
        let mut name = String::from(s.trim_end());
        println!("{}", name.len());
        if(name.len() > 1) {
            let mut file_handler: File;

            match File::open(&name) {
                Ok(opened) => {
                    file_handler = opened;
                    println!("Opening handler for file: {}", &name);

                    Some(Self {
                        file_handler,
                        items: Vec::<Item>::new(),
                    })
                }
                Err(e) => {
                    panic!("Database does not exist. If you wish to create a new database, please use the create function.");
                }
            }

        } else {
            panic!("Error! Failed to enumerate database.")
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
