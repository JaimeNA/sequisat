mod propagator;
mod tle;
mod orbit;
mod satellite;

use satellite::Satellite;
use std::{io, thread};
use std::time::{Duration, Instant};

use ratatui::symbols;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph},
    widgets::canvas::{Canvas, Context, Map, MapResolution, Rectangle, Points},
    text::{Span},
    text,
    layout::{Constraint, Rect, Direction, Layout},
    Frame,
    Terminal
};

use crossterm::event;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen},
    event::{Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
};

fn ui(f: &mut Frame, sat: &Satellite) {
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

fn paint_map(ctx: &mut Context, sat: &Satellite)
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

    ctx.layer();

    ctx.draw(&Points {
        coords: sat.get_points(),
        color: Color::Green
    });
}


fn draw_coords(f: &mut Frame, chunk: Rect, sat: &Satellite)
{
    let coords = Block::default()
        .title("Coordinates")
        .borders(Borders::ALL);

    let text = vec![
        text::Line::from(vec![
            Span::from("Altitude: "),
            Span::styled(sat.getAltitude().to_string(), Style::default().fg(Color::Red)),
        ]),
        text::Line::from(vec![
            Span::from("Latitude: "),
            Span::styled((sat.getLatitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Blue)),
        ]),
        text::Line::from(vec![
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

    let mut noaa_18 = Satellite::new("iss.tle");
    noaa_18.print();    // TODO: Implement to_string
    
    // Set the update interval (e.g., 1 second)
    let update_interval = Duration::from_secs(1);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate: Duration = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    // Start the continuous update loop
    loop {

        let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(q) => break,
                    _ => {}
                }
            }
        }
        
        if last_tick.elapsed() >= tick_rate {
            noaa_18.get_trajectory();

            noaa_18.update_position();

            terminal.draw(|f| ui(f, &noaa_18))?;
            last_tick = Instant::now();
        }
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


    

