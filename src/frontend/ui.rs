use crate::App;

use ratatui::{
    style::{Style, Color, Modifier},
    widgets::{Borders, Block, Paragraph, Tabs},
    widgets::canvas::{Canvas, Points, Circle, MapResolution, Map, Context},
    prelude::{Constraint, Rect, Direction, Layout},
    text::Span,
    symbols,
    text,
    Frame
};

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
const RED: Color = Color::Rgb(215, 0, 0);
const BLACK: Color = Color::Rgb(8, 8, 8); // not really black, often #080808
const DARK_GRAY: Color = Color::Rgb(68, 68, 68);
const MID_GRAY: Color = Color::Rgb(128, 128, 128);
const LIGHT_GRAY: Color = Color::Rgb(188, 188, 188);
const WHITE: Color = Color::Rgb(238, 238, 238); // not really white, often #eeeeee

pub fn draw(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let [title_bar, tab, bottom_bar] = vertical.areas(frame.area());
 
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .divider("")
        .padding("", "")
        .highlight_style(Style::new()
            .fg(WHITE)
            .bg(DARK_BLUE)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED))
        .select(app.tabs.index);

     frame.render_widget(tabs, title_bar);
     draw_map_tab(frame, app, tab);
}

fn draw_map_tab(frame: &mut Frame, app: &mut App, area: Rect)
{
    let chunks = Layout::default()
         .direction(Direction::Horizontal)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(80),
                 Constraint::Percentage(20)
             ].as_ref()
         )
         .split(area);
 
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
                 Constraint::Percentage(15),
                 Constraint::Percentage(85)
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
         x: (app.sat.get_longitude()* 180.0/3.14159),
         y: (app.sat.get_latitude()* 180.0/3.14159),
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
             Span::styled(format!("{:.5} km", app.sat.get_altitude().to_string()), Style::default().fg(Color::Red)),
         ]),
         text::Line::from(vec![
             Span::from("Latitude: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_latitude() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
         ]),
         text::Line::from(vec![
             Span::from("Longitud: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_longitude() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
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
             Span::from("Satellite Catalog Number: "),
             Span::styled(app.sat.get_tle().get_catalog_number().to_string(), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Classification: "),
            Span::styled(app.sat.get_tle().get_classification(), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Launch Year: "),
            Span::styled(app.sat.get_tle().get_launch_year().to_string(), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Launch Piece: "),
            Span::styled(app.sat.get_tle().get_launch_piece().to_string(), Style::default().fg(Color::Yellow)),
         ]),
         // TODO: change color of smth to mark them as different
         text::Line::from(vec![
            Span::from("Epoch Year: "),
            Span::styled(app.sat.get_tle().get_epoch_year().to_string(), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Epoch Day of Year: "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_epoch_day().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Ballistic Coefficient: "),
            Span::styled(format!("{:.10}", app.sat.get_tle().get_ballistic_coefficient()).to_string(), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Drag Term: "),
            Span::styled(format!("{:.10}", app.sat.get_tle().get_drag_term().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Inclination(rads): "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_inclination().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Right Ascension(rads): "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_right_ascension().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Eccentricity: "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_eccetricity().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Argument of Perigee(rads): "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_argument_of_perigee().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Mean Anomaly(rads): "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_mean_anomaly().to_string()), Style::default().fg(Color::Yellow)),
         ]),
         text::Line::from(vec![
            Span::from("Mean Motion(rads/min): "),
            Span::styled(format!("{:.5}", app.sat.get_tle().get_mean_motion().to_string()), Style::default().fg(Color::Yellow)),
         ]),
     ];
 
     let data = Paragraph::new(text)
         .block(tle_data)
         .style(Style::default().fg(Color::White));
 
     frame.render_widget(data, area); 
 }

// TODO: check good practices for functions visibility