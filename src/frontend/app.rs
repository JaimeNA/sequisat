use crate::Satellite;

use ratatui::widgets::ListState;

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
}   

impl<'a> App<'a> {
    pub fn new(title: &'a str, sat: Satellite) -> Self{
        Self {
            title,
            sat,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            should_quit: false,
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

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.sat.get_trajectory();
        self.sat.update_position();
    }
    
}