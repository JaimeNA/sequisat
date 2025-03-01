use crate::Satellite;

use crate::PositionVector;

use ratatui::{
    crossterm::event::KeyCode,
    widgets::ListState
};

use std::{
    fs,
    fs::ReadDir
};

pub enum MessageType {
    Error,
    Warning,
    Info
}

impl MessageType {
    pub fn name(&self) -> &'static str {
        match self {
            MessageType::Error => "Error",
            MessageType::Warning => "Warning",
            MessageType::Info => "Info",
        }
    }
}

pub struct Message {
    msg_type: MessageType,
    msg: String
}

impl Message {
    pub fn new(msg_type: MessageType, msg: String) -> Self {
        Self{
            msg_type: msg_type,
            msg: msg
        }
    }

    pub fn get_type(&self) -> &MessageType
    {
        &self.msg_type
    } 

    pub fn get_message(&self) -> &String
    {
        &self.msg
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// Struct taken from the examples in the ratatui repository
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>
}

impl<T> StatefulList<T> where T : Clone {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            items,
        }
    }

    pub fn next(&mut self) -> &T{
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        
        &self.items[i]
    }

    pub fn previous(&mut self) -> &T {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        &self.items[i]
    }

    pub fn get_selected(&self) -> &T {
        let i = match self.state.selected() {
            Some(i) => {
                i
            }
            None => 0,
        };
        &self.items[i]
    }
}

pub struct App<'a> {  // TODO: Make em private
    pub title: &'a str,
    pub sat: Option<Satellite>,
    pub tle_list: StatefulList<String>,
    pub tabs: TabsState<'a>,
    pub should_quit: bool,
    pub usr_geodetic: PositionVector,
    pub input_mode: bool,
    pub buffer: String,
    messages: Vec<Message>
}   

impl<'a> App<'a> {

    const DEF_LAT: f64 = -34.603599 * (core::f64::consts::PI/180.0);
    const DEF_LON: f64 = -58.381555 * (core::f64::consts::PI/180.0); // Buenos Aires, Argentina

    const INPUT_ARG_ERROR: &'static str = "Invalid number of arguments";
    const INPUT_TYPE_ERROR: &'static str = "Invalid type";

    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            sat: None,
            tle_list: StatefulList::new(Self::get_tle_files()),
            tabs: TabsState::new(vec!["Map Projection", "Azimuthal Projection", "About"]),
            should_quit: false,
            usr_geodetic: PositionVector::new(Self::DEF_LAT, Self::DEF_LON, 0.0),
            input_mode: false,
            buffer: String::new(),
            messages: Vec::new(),
        }
    }

    fn pop_message(&mut self) {
        if !self.messages.is_empty() {
            self.messages.pop();
        }
    }

    fn push_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn get_sat(&self) -> &Satellite {
        &self.sat
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn on_up(&mut self) {
        self.sat = Satellite::new(self.tle_list.previous());
    }

    pub fn on_down(&mut self) {
        self.sat = Satellite::new(self.tle_list.next());
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key_normal(&mut self, c: KeyCode) {
        match c {
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            KeyCode::Char('c') => {
                self.input_mode = true;
            },
            KeyCode::Char('f') => {
               
                self.push_message(Message::new(MessageType::Info, Self::get_tle_files().last().unwrap().to_string()));
            },
            KeyCode::Enter => self.pop_message(),
            _ => {}
        }
    }

    pub fn on_key_input(&mut self, c: KeyCode) {
        match c {
            KeyCode::Enter => {
                // Process the current buffer
                let result = Self::text_to_coordinates(self.buffer.clone());

                // Return to normal mode
                self.visual_mode();

                // Check for errors
                if  let Err(e) = result {
                    self.push_message(Message::new(MessageType::Error, e));
                } else {
                    self.usr_geodetic = result.unwrap();
                }
            },
            KeyCode::Backspace => {
                self.buffer.pop();
            },
            KeyCode::Char(c) => self.buffer.push(c),
            _ => {}
        }
    }

    fn get_error_msg(msg: &str) -> String {
        format!("ERROR::APP: {}", msg)
    }

    fn text_to_coordinates(text: String) -> Result<PositionVector, String> {
        let mut columns = text.split_whitespace();

        let mut input = columns.next();

        // Get latitude
        if input.is_none()
        {    return Err(Self::get_error_msg(Self::INPUT_ARG_ERROR));   }
        let mut value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return Err(Self::get_error_msg(Self::INPUT_TYPE_ERROR));   }
        let lat = value.unwrap() * (core::f64::consts::PI/180.0);

        input = columns.next();

        // Get longitude
        if input.is_none()
        {    return Err(Self::get_error_msg(Self::INPUT_ARG_ERROR));   }
        value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return Err(Self::get_error_msg(Self::INPUT_TYPE_ERROR));   }
        let lon = value.unwrap() * (core::f64::consts::PI/180.0);
        
        input = columns.next();

        // Get altitude
        if input.is_none()
        {    return Err(Self::get_error_msg(Self::INPUT_ARG_ERROR));   }
        value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return Err(Self::get_error_msg(Self::INPUT_TYPE_ERROR));   } 
        let alt = value.unwrap();

        Ok(PositionVector::new(lat, lon, alt))
    }

    fn get_tle_files() -> Vec<String> {
        let mut tles = Vec::new();
    
        // Get the current directory
        let current_dir = ".";

        // Read the entries in the current directory
        for entry in fs::read_dir(current_dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry"); // TODO: Error handling
            
            // Check if it's a file and if it ends with .tle
            if entry.path().is_file() && entry.path().extension().map(|ext| ext == "tle").unwrap_or(false) {
                tles.push(entry.file_name().to_string_lossy().to_string());
            }
        }

        tles
    }

    fn visual_mode(&mut self) { 
        self.input_mode = false;
        self.buffer.clear();
    }

    pub fn on_tick(&mut self) {
        self.sat.get_trajectory();
        self.sat.update_position();
    }
    
}
