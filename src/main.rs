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

use tui::widgets::GraphType;
use tui::widgets::Dataset;
use tui::symbols::Marker;
use tui::widgets::Chart;
use tui::style::Color;
use tui::widgets::Axis;
use tui::text::Span;
use tui::style::Style;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::Paragraph;
use tui::layout::Rect;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    draw_coords(f, chunks[0]);

    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn draw_coords<B: Backend>(f: &mut Frame<B>, chunk: Rect)
{
    let coords = Block::default()
        .title("Coordinates")
        .borders(Borders::ALL);

    let altitude = Paragraph::new(format!("Altitude: {}", getAltitude()))
        .block(coords)
        .style(Style::default().fg(Color::White));

    f.render_widget(altitude, chunk);
}

fn getAltitude() -> f64
{
    return 65.0;
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

    let mut test_coord: Vec<(f64, f64)> = Vec::new();

    for hours in 0..240 {
        println!("t = {} min", hours * 60);
        
        test_coord.push(((hours*60) as f64, iss.update_gravity_and_atm_drag((hours * 60) as f64).sin()));
        //println!("    ṙ = {:?} km.s⁻¹", prediction.velocity);
    }
    
    // Set the update interval (e.g., 1 second)
    let update_interval = Duration::from_secs(1);

    // Start the continuous update loop
    //loop {
        // Get the current time (UTC)
    //    let current_time = Utc::now().naive_utc();

        // Calculate the time since the epoch in minutes
    //    let time_since_epoch = time_since_epoch_in_minutes(epoch_year, epoch_day);

        // Display the result
    //    println!("Time since epoch: {} minutes", time_since_epoch);
    //    iss.update_gravity_and_atm_drag(time_since_epoch);

        // Wait for the update interval
    //    thread::sleep(update_interval);
    //}
    

    // ---- EXPERIMENTAL -----

        

    let datasets = vec![
        Dataset::default()
            .name("data1")
            .marker(Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&test_coord),
    ];
    let _chart: Chart = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 2400.0])
            .labels(["0.0", "500.0", "1000.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 1.5])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

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
        let tle_date = NaiveDate::from_yo(epoch_year, day_of_year);

        // Calculate the time from the fractional day part (fraction of 24 hours)
        let seconds_in_day = 86400.0 * (epoch_day - day_of_year as f64);
        let tle_time = NaiveTime::from_num_seconds_from_midnight(seconds_in_day as u32, 0);

        // Create a full TLE epoch DateTime in UTC
        let tle_datetime = Utc
            .from_utc_datetime(&NaiveDate::and_time(&tle_date, tle_time))
            .with_timezone(&Utc);

        // Get the current time in UTC
        let now = Utc::now();

        // Calculate the delta in minutes
        let delta = now.signed_duration_since(tle_datetime).num_seconds() as f64 / 60.0;

        delta
    }

