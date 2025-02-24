mod backend;
mod frontend;

use frontend::{app::App, ui};

use backend::satellite::Satellite;
use backend::vector::PositionVector;

use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::Backend,
    style::Color,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
    },
    Terminal,
};

pub fn run(tick_rate: Duration) -> io::Result<()> {

    let mut terminal = ratatui::init();
    // create app and run it
    let app = App::new("SEQUISAT", Satellite::new("noaa.tle"));
    let app_result = run_app(&mut terminal, app, tick_rate);

    ratatui::restore();

    app_result
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (app.input_mode, key.code) {
                        (false, KeyCode::Left | KeyCode::Char('h')) => app.on_left(),
                        (false, KeyCode::Up | KeyCode::Char('k')) => app.on_up(),
                        (false, KeyCode::Right | KeyCode::Char('l')) => app.on_right(),
                        (false, KeyCode::Down | KeyCode::Char('j')) => app.on_down(),
                        (false, _) => app.on_key_normal(key.code),
                        (true, _) => app.on_key_input(key.code)
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    
    // Set the update interval (e.g., 1 second)
    let update_interval = Duration::from_secs(1);

    let tick_rate: Duration = Duration::from_millis(100);
    
    run(tick_rate)?;

    Ok(())
}


    

