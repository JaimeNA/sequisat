
use backend::satellite::Satellite;

pub struct App {
    pub sat: Satellite,
    pub should_quit: bool,
}   

impl App {
    pub fn new(sat: Satellite) {
        Self {
            sat: sat,
            should_quit: false,
        }
    }

}