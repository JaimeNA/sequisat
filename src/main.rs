mod backend;
mod frontend;

use frontend::{app::App, ui};

use backend::satellite::Satellite;
use std::thread;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::Color,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> io::Result<()> {

    // // create app and run it
    // let app = App::new(Satellite::new("noaa.tle"));
    // let app_result = run_app(&mut terminal, app, tick_rate);

    let mut terminal = ratatui::init();
    let app_result = App::new(Satellite::new("noaa.tle")).run(&mut terminal);
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
                match key.code {
                    KeyCode::Char(q) => break,
                    _ => {}
                }
            }
        }

        // if event::poll(timeout)? {
        //     if let Event::Key(key) = event::read()? {
        //         if key.kind == KeyEventKind::Press {
        //             match key.code {
        //                 KeyCode::Left | KeyCode::Char('h') => app.on_left(),
        //                 KeyCode::Up | KeyCode::Char('k') => app.on_up(),
        //                 KeyCode::Right | KeyCode::Char('l') => app.on_right(),
        //                 KeyCode::Down | KeyCode::Char('j') => app.on_down(),
        //                 KeyCode::Char(c) => app.on_key(c),
        //                 _ => {}
        //             }
        //         }
        //     }
        // }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.exit {
            return Ok(());
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {

    let mut noaa_18 = Satellite::new("noaa.tle");
    noaa_18.print();    // TODO: Implement to_string
    
    // Set the update interval (e.g., 1 second)
    let update_interval = Duration::from_secs(1);

    let tick_rate: Duration = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    // Start the continuous update loop
    // loop {

    //     let timeout = tick_rate
    //     .checked_sub(last_tick.elapsed())
    //     .unwrap_or_else(|| Duration::from_secs(0));

    //     if crossterm::event::poll(timeout)? {
    //         if let Event::Key(key) = event::read()? {
    //             match key.code {
    //                 KeyCode::Char(q) => break,
    //                 _ => {}
    //             }
    //         }
    //     }
        
    //     if last_tick.elapsed() >= tick_rate {
    //         noaa_18.get_trajectory();

    //         noaa_18.update_position();

    //         terminal.draw(|f| ui(f, &noaa_18))?;
    //         last_tick = Instant::now();
    //     }
    // }
    
    run(tick_rate, true)?;

    Ok(())
}


    

