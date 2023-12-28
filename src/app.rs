use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, read_to_string};
use std::ops::Add;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
}


pub struct App {
    pub key_input: String,
    pub key_buffer: Vec<String>,
    pub file_path: String,
    pub current_screen: CurrentScreen,
    pub current_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App{
            key_input: String::new(),
            key_buffer: vec![],
            file_path: String::new(),
            //file_name: String::from("New File"),
            current_screen: CurrentScreen::Main,
            current_editing: None,
        }
    }

    pub fn save_key_values(&mut self) {
        self.key_buffer.push(self.key_input.clone());
        self.key_input = String::new();
        self.current_editing = None;
    }

    pub fn write_txt(&self) -> Result<(), Error>{

        let mut output = File::create(&self.file_path)?;
        let mut all_string = String::new();
        for string in &self.key_buffer {
            all_string.add(&string[..]);
        }
        output.write_all(all_string.as_bytes())?;
        Ok(())

    }
}