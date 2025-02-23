use crate::Satellite;

use crate::PositionVector;

use ratatui::{
    widgets::ListState,
    crossterm::event::KeyCode
};

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

pub struct App<'a> {
    pub title: &'a str,
    pub sat: Satellite,
    pub tabs: TabsState<'a>,
    pub should_quit: bool,
    pub usr_geodetic: PositionVector,
    pub input_mode: bool,
    pub buffer: String,
}   

impl<'a> App<'a> {

    const DEF_LAT: f64 = -34.603599 * (core::f64::consts::PI/180.0);
    const DEF_LON: f64 = -58.381555 * (core::f64::consts::PI/180.0); // Buenos Aires, Argentina

    const INPUT_ARG_ERROR: &'static str = "Invalid number of arguments";
    const INPUT_TYPE_ERROR: &'static str = "Invalid type";

    pub fn new(title: &'a str, sat: Satellite) -> Self{
        Self {
            title,
            sat,
            tabs: TabsState::new(vec!["Map Projection", "Azimuthal Projection", "About"]),
            should_quit: false,
            usr_geodetic: PositionVector::new(Self::DEF_LAT, Self::DEF_LON, 0.0),
            input_mode: false,
            buffer: String::new()
        }
    }

    pub fn on_up(&mut self) {
        //self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        //self.tasks.next();
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
            _ => {}
        }
    }

    pub fn on_key_input(&mut self, c: KeyCode) -> Result<(), &str> {
        match c {
            KeyCode::Enter => {
                let result = self.get_user_coordinates();

                if let Err(e) = result {
                    return Err(e);
                }
                
                self.usr_geodetic = result;
            },
            KeyCode::Backspace => {
                self.buffer.pop();
            },
            KeyCode::Char(c) => self.buffer.push(c),
            _ => {}
        }

        Ok(())
    }

    fn get_error_msg(msg: &str) -> &str {
        "format!(ERROR::APP: {}, msg).clone().as_str()"
    }

    pub fn get_user_coordinates(&mut self) -> Result<PositionVector, &str> { // TODO: use lifetime
        let mut columns = self.buffer.split_whitespace();

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

        self.visual_mode();
        
        Ok(PositionVector::new(lat, lon, alt))
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
