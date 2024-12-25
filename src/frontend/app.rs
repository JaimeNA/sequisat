use crate::Satellite;

use std::io;

use ratatui::{
    style::{Style, Color},
    widgets::{Widget, Borders, Block, Paragraph},
    widgets::canvas::{Canvas, Points, Circle, MapResolution, Map, Context},
    prelude::{Constraint, Rect, Direction, Layout},
    text::{Text, Line, Span},
    prelude::{Buffer, Stylize},
    symbols::border,
    DefaultTerminal,
    text,
    Frame
};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, Event};
use crossterm::event;

pub struct App {
    pub sat: Satellite,
    pub exit: bool,
}   

impl App {
    pub fn new(sat: Satellite) -> App{
        Self {
            sat: sat,
            exit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.sat.get_trajectory();
        self.sat.update_position();
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
           " self.counter.to_string()".yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}