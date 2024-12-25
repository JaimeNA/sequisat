use crate::Satellite;

pub struct App {
    pub sat: Satellite,
    pub should_quit: bool,
}   

impl App {
    pub fn new(sat: Satellite) -> App{
        Self {
            sat: sat,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.sat.get_trajectory();
        self.sat.update_position();
    }
    
}