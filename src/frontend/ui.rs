use crate::App;

use crate::Vector3;

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
 
    draw_title_bar(frame, app, title_bar);

    match app.tabs.index {
        0 => draw_map_tab(frame, app, tab),
        1 => draw_azimuth_tab(frame, app, tab),
        2 => draw_tle_data(frame, app, tab),
        _ => {}
    };
}

fn draw_title_bar(frame: &mut Frame, app: &mut App, area: Rect)
{
    let layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(43)]);
    let [title_area, tabs_area] = layout.areas(area);

    let title = Span::styled(app.title, Style::new()
        .fg(WHITE)
        .add_modifier(Modifier::BOLD));

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .divider("|")
        .padding("", "")
        .highlight_style(Style::new()
            .fg(WHITE)
            .bg(DARK_BLUE)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED))
        .select(app.tabs.index);

    frame.render_widget(title, title_area);
    frame.render_widget(tabs, tabs_area);
}

fn draw_map_tab(frame: &mut Frame, app: &mut App, area: Rect)
{
    let chunks = Layout::default()
         .direction(Direction::Horizontal)
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
     draw_sat_coords(frame, app, chunklin[0]);
     draw_user_coords(frame, app, chunklin[1]);
}

fn draw_azimuth_tab(frame: &mut Frame, app: &mut App, area: Rect)
{

    let map = Canvas::default()
    .block(Block::default().title("Azimuth").borders(Borders::ALL))
    .paint(|ctx| paint_azimuth(ctx, app))
    .marker(symbols::Marker::Braille)
    .x_bounds([-180.0, 180.0])
    .y_bounds([-180.0, 180.0]);

    frame.render_widget(map, area);    
}

 fn paint_map(ctx: &mut Context, app: &App)
 {
     
     ctx.draw(&Map {
         color: Color::White,
         resolution: MapResolution::High,
     });

     ctx.layer();    
     
     ctx.draw(&Circle {
         x: get_user_location().get_y(),
         y: get_user_location().get_z(),
         radius: 1.0,
         color: Color::Red,
     });
 
     ctx.layer();
 
     ctx.draw(&Points {
         coords: app.sat.get_points(),
         color: Color::Green
     });

     ctx.layer();    // Go one layer above
                     //
     ctx.draw(&Circle {
         x: (app.sat.get_longitude()* 180.0/3.14159),
         y: (app.sat.get_latitude()* 180.0/3.14159),
         radius: 5.0,
         color: Color::Yellow,
     });
 }
 
fn draw_user_coords(frame: &mut Frame, app: &mut App, area: Rect)
{

    let position_data = Block::default()
    .title("User Coordinates")
    .borders(Borders::ALL);

    let text = vec![
        text::Line::from(vec![
            Span::from("Longitude: "),
            Span::styled(format!("{:.5} deg", get_user_location().get_y().to_string()), Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("Latitude: "),
            Span::styled(format!("{:.5} deg", get_user_location().get_z().to_string()), Style::default().fg(Color::Blue)),
        ])
    ];

    let data = Paragraph::new(text)
        .block(position_data)
        .style(Style::default().fg(Color::White));

    frame.render_widget(data, area);   
}

 fn draw_sat_coords(frame: &mut Frame, app: &mut App, area: Rect) // Repeated code, fix later
 {
     let position_data = Block::default()
         .title("Satellite Coordinates")
         .borders(Borders::ALL);
 
     let text = vec![
         text::Line::from(vec![
             Span::from("Altitude: "),
             Span::styled(format!("{:.5} km", app.sat.get_altitude().to_string()), Style::default().fg(Color::Red)),
         ]),
         text::Line::from(vec![
             Span::from("Longitude: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_longitude() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
         ]),
         text::Line::from(vec![
             Span::from("Latitude: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_latitude() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
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


fn paint_azimuth(ctx: &mut Context, app: &App)
{
    ctx.draw(&Circle {
        x: 0.0,
        y: 0.0,
        radius: 90.0,
        color: Color::Yellow,
    });

    ctx.layer();

    let omega = -58.381555 * (core::f64::consts::PI/180.0);
    let phi = -34.603599 * (core::f64::consts::PI/180.0);

    let usr = Vector3::new(omega.cos()*phi.sin(), omega.sin()*phi.sin(), phi.cos());
    let mut sat = app.sat.get_position().clone();

    sat.sub(&usr); // TODO: Make another vector type thats just for spheric coordinates

    let mut sph_diff = cartesian_to_spheric(sat);

    let sph_usr = cartesian_to_spheric(usr);

    sph_diff.sum(&sph_usr);
    
    ctx.draw(&Circle {
        x: (sph_diff.get_y() * 180.0/3.14159),
        y: ((sph_diff.get_z().powi(2) - sph_diff.get_y().powi(2)).sqrt() * 180.0/3.14159),
        radius: 5.0,
        color: Color::Blue,
    });
}
// TODO: check good practices for functions visibility

// EXPERIMENTAL - Will move later

fn get_user_location() -> Vector3 // Radius, Longitude and Altitude
{
    Vector3::new(0.0, -58.381555, -34.603599)
}

fn spheric_to_cartesian(position: Vector3) -> Vector3
{
    Vector3::new(
        position.get_x()*position.get_y().cos()*position.get_z().sin(), 
        position.get_x()*position.get_y().sin()*position.get_z().sin(), 
        position.get_x()*position.get_z().cos()
    )
}

fn cartesian_to_spheric(position: Vector3) -> Vector3
{
    let radius = (position.get_x().powi(2) + position.get_y().powi(2) + position.get_z().powi(2)).sqrt();
    let longitude = position.get_y().atan2(position.get_x());


    let latitude = (position.get_z() / (position.get_x().powi(2) + position.get_y().powi(2)).sqrt()).atan();

    Vector3::new(radius, longitude, latitude)
}