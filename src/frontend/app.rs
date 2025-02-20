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

    pub fn on_key_input(&mut self, c: KeyCode) {
        match c {
            KeyCode::Enter => {
                self.set_user_coordinates();
            },
            KeyCode::Char(c) => self.buffer.push(c),
            _ => {}
        }
    }

    pub fn set_user_coordinates(&mut self) {
        let mut columns = self.buffer.split_whitespace();

        let mut input = columns.next();

        // Get latitude
        if input.is_none()
        {    return;   } // TODO: Error handling
        let mut value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return;   } // TODO: Error handling
        let lat = value.unwrap() * (core::f64::consts::PI/180.0);

        input = columns.next();

        // Get longitude
        if input.is_none()
        {    return;   } // TODO: Error handling
        value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return;   } // TODO: Error handling
        let lon = value.unwrap() * (core::f64::consts::PI/180.0);
        
        input = columns.next();

        // Get altitude
        if input.is_none()
        {    return;   } // TODO: Error handling
        value = input.unwrap().parse::<f64>();

        if value.is_err()
        {    return;   } // TODO: Error handling
        let alt = value.unwrap();

        self.usr_geodetic = PositionVector::new(lat, lon, alt);

        self.input_mode = false;
        self.buffer.clear();
    }

    pub fn on_tick(&mut self) {
        self.sat.get_trajectory();
        self.sat.update_position();
    }
    
}