use crate::App;
use crate::Satellite;

use ratatui::{
    style::{Style, Color},
    widgets::{Borders, Block, Paragraph},
    widgets::canvas::{Canvas, Points, Circle, MapResolution, Map, Context},
    prelude::{Constraint, Rect, Direction, Layout},
    text::{Line, Span},
    symbols,
    text,
    Frame
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
         .direction(Direction::Horizontal)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(80),
                 Constraint::Percentage(20)
             ].as_ref()
         )
         .split(frame.size());
 
     let map = Canvas::default()
         .block(Block::default().title("World").borders(Borders::ALL))
         .paint(|ctx| paint_map(ctx, app))
         .marker(symbols::Marker::Braille)
         .x_bounds([-180.0, 180.0])
         .y_bounds([-90.0, 90.0]);
    
    frame.render_widget(map, chunks[0]);


    let chunklin = Layout::default()
         .direction(Direction::Vertical)
         .constraints(
             [
                 Constraint::Percentage(10),
                 Constraint::Percentage(90)
             ].as_ref()
         )
         .split(chunks[1]);
     draw_coords(frame, app, chunklin[0]);
     draw_tle_data(frame, app, chunklin[1]);
}
 
 fn paint_map(ctx: &mut Context, app: &App)
 {
     
     ctx.draw(&Map {
         color: Color::White,
         resolution: MapResolution::High,
     });
     ctx.layer();    // Go one layer above
                     //
     ctx.draw(&Circle {
         x: (app.sat.getLongitude()* 180.0/3.14159),
         y: (app.sat.getLatitude()* 180.0/3.14159),
         radius: 5.0,
         color: Color::Yellow,
     });
 
     ctx.layer();
 
     ctx.draw(&Points {
         coords: app.sat.get_points(),
         color: Color::Green
     });
 }
 
 
 fn draw_coords(frame: &mut Frame, app: &mut App, area: Rect)
 {
     let position_data = Block::default()
         .title("Coordinates")
         .borders(Borders::ALL);
 
     let text = vec![
         text::Line::from(vec![
             Span::from("Altitude: "),
             Span::styled(app.sat.getAltitude().to_string(), Style::default().fg(Color::Red)),
         ]),
         text::Line::from(vec![
             Span::from("Latitude: "),
             Span::styled((app.sat.getLatitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Blue)),
         ]),
         text::Line::from(vec![
             Span::from("Longitud: "),
             Span::styled((app.sat.getLongitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Green)),
         ])
     ];
 
     let data = Paragraph::new(text)
         .block(position_data)
         .style(Style::default().fg(Color::White));
 
     frame.render_widget(data, area); 
 }

 fn draw_tle_data(frame: &mut Frame, app: &mut App, area: Rect)
 {
     let tle_data = Block::default()
         .title("TLE Data")
         .borders(Borders::ALL);
 
     let text = vec![
         text::Line::from(vec![
             Span::from("Altitude: "),
             Span::styled(app.sat.getAltitude().to_string(), Style::default().fg(Color::Red)),
         ]),
         text::Line::from(vec![
             Span::from("Latitude: "),
             Span::styled((app.sat.getLatitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Blue)),
         ]),
         text::Line::from(vec![
             Span::from("Longitud: "),
             Span::styled((app.sat.getLongitude() * (180.0/core::f64::consts::PI)).to_string(), Style::default().fg(Color::Green)),
         ])
     ];
 
     let data = Paragraph::new(text)
         .block(tle_data)
         .style(Style::default().fg(Color::White));
 
     frame.render_widget(data, area); 
 }