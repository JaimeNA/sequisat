mod sgp4_propagator;
mod sgp4;

use chrono::prelude::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};

use sgp4_propagator::TLE;
use sgp4_propagator::Orbit;

use std::{io, thread, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, canvas::*},
    Terminal,
};

use tui::layout::Rect;
use tui::widgets::canvas::Line;  
use tui::symbols;

use tui::{
    backend::Backend,
    text::{Span, Spans, Text},
    Frame,
};

fn ui<B: Backend>(f: &mut Frame<B>, sat: &sgp4::SGP4) {
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref()
        )
        .split(f.size());

    draw_coords(f, chunks[0], sat);

    let map = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .paint(|ctx| paint_map(ctx, sat))
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);

    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(map, chunks[1]);
}

fn paint_map(ctx: &mut Context, sat: &sgp4::SGP4)
{
    
    ctx.draw(&Map {
        color: Color::White,
        resolution: MapResolution::High,
    });
    ctx.layer();    // Go one layer above
                    //
    ctx.draw(&Rectangle {
        x: (sat.getLongitude()* 180.0/3.14159),
        y: (sat.getLatitude()* 180.0/3.14159),
        width: 10.0,
        height: 10.0,
        color: Color::Yellow,
    });

}


fn draw_coords<B: Backend>(f: &mut Frame<B>, chunk: Rect, sat: &sgp4::SGP4)
{
    let coords = Block::default()
        .title("Coordinates")
        .borders(Borders::ALL);

    let text = vec![
        Spans::from(vec![
            Span::from("Altitude: "),
            Span::styled(sat.getAltitude().to_string(), Style::default().fg(Color::Red)),
        ]),
        Spans::from(vec![
            Span::from("Latitude: "),
            Span::styled((sat.getLatitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Blue)),
        ]),
        Spans::from(vec![
            Span::from("Longitud: "),
            Span::styled((sat.getLongitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Green)),
        ])
    ];

    let altitude = Paragraph::new(text)
        .block(coords)
        .style(Style::default().fg(Color::White));


   // let latitude = Paragraph::new(format!("Latitude: {}", sat.getLatitude()))
   //     .block(coords)
   //     .style(Style::default().fg(Color::White));

    //let longitude = Paragraph::new(format!("Longitude: {}", sat.getLongitude()))
    //    .block(coords)
    //    .style(Style::default().fg(Color::White));

    f.render_widget(altitude, chunk); 
    //f.render_widget(latitude, chunk);
    //f.render_widget(longitude, chunk);
}


fn main() -> Result<(), io::Error> {

    let tle = TLE::from_file("noaa.tle");
    tle.print_data();
    let epoch_year = tle.epoch_year;
    let epoch_day = tle.epoch_day;

    let orbit_0 = Orbit::from_tle(tle);
    let mut iss = sgp4::SGP4::new(orbit_0);

    iss.calculate_constants();

    iss.print_data();

    // Set the update interval (e.g., 1 second)
    let update_interval = Duration::from_secs(1);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start the continuous update loop
    loop {
        // Calculate the time since the epoch in minutes
        let time_since_epoch = time_since_epoch_in_minutes(epoch_year, epoch_day);

        // Display the result
        iss.update_gravity_and_atm_drag(time_since_epoch);

        terminal.draw(|f| ui(f, &iss))?;

        // Poll for events and check if 'q' key is pressed
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // Exit the loop when 'q' is pressed
                }
            }
        }

        // Wait for the update interval
        thread::sleep(update_interval);
    }
    

    // ---- EXPERIMENTAL -----

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


    // Function to calculate the time difference between two NaiveDateTime in minutes
    pub fn time_since_epoch_in_minutes(epoch_year :i32, epoch_day :f64) -> f64 {

        let day_of_year = epoch_day as u32;

        // Convert the day of the year to a NaiveDate
        let tle_date = NaiveDate::from_yo_opt(epoch_year, day_of_year);

        // Calculate the time from the fractional day part (fraction of 24 hours)
        let seconds_in_day = 86400.0 * (epoch_day - day_of_year as f64);
        let tle_time = NaiveTime::from_num_seconds_from_midnight_opt(seconds_in_day as u32, 0);

        // Create a full TLE epoch DateTime in UTC
        let tle_datetime = Utc
            .from_utc_datetime(&NaiveDate::and_time(&tle_date.unwrap(), tle_time.unwrap()))
            .with_timezone(&Utc);

        // Get the current time in UTC
        let now = Utc::now();

        // Calculate the delta in minutes
        let delta = now.signed_duration_since(tle_datetime).num_seconds() as f64 / 60.0;

        delta
    }

