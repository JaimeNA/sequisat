use crate::App;

use crate::Vector3;

use ratatui::{
    style::{Style, Color, Modifier},
    widgets::{Borders, Block, Paragraph, Tabs},
    widgets::canvas::{Canvas, Points, Circle, Line, MapResolution, Map, Context},
    prelude::{Constraint, Rect, Direction, Layout},
    text::Span,
    symbols,
    text,
    Frame
};

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
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
         x: (app.sat.get_geodetic_position().get_y()* 180.0/3.14159),
         y: (app.sat.get_geodetic_position().get_x()* 180.0/3.14159),
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
             Span::styled(format!("{:.5} km", app.sat.get_geodetic_position().get_z().to_string()), Style::default().fg(Color::Red)),
         ]),
         text::Line::from(vec![
             Span::from("Longitude: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_geodetic_position().get_y() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
         ]),
         text::Line::from(vec![
             Span::from("Latitude: "),
             Span::styled(format!("{:.5} deg", (app.sat.get_geodetic_position().get_x() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
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

    // draw grid<
    ctx.draw(&Circle {
        x: 0.0,
        y: 0.0,
        radius: 90.0,
        color: Color::Yellow,
    });

    ctx.draw(&Circle {
        x: 0.0,
        y: 0.0,
        radius: 45.0,
        color: Color::Red,
    });

    ctx.draw(&Line {
        x1: 90.0,
        y1: 0.0,
        x2: -90.0,
        y2: 0.0,
        color: Color::Red,
    });

    ctx.draw(&Line {
        x1: 0.0,
        y1: 90.0,
        x2: 0.0,
        y2: -90.0,
        color: Color::Red,
    });

    // draw markers
    ctx.print(0.0, -90.0, "0");
    ctx.print(0.0, -45.0, "45");
    ctx.print(0.0, 0.0, "90");

    ctx.print(0.0, 90.0, "N");
    ctx.print(90.0, 0.0, "E");
    ctx.print(-90.0, 0.0, "W");


    // Draw sat
    let lon = -58.381555 * (core::f64::consts::PI/180.0);
    let lat = -34.603599 * (core::f64::consts::PI/180.0);

    let usr_geodetic = Vector3::new(lat, lon, 0.0);
    let sat_geodetic = app.sat.get_geodetic_position();

    let usr_ecef = usr_geodetic.geodetic_to_ecef();
    let sat_ecef = sat_geodetic.geodetic_to_ecef();
    
    let p_enu = Vector3::ecef_to_enu(&usr_ecef, &sat_ecef);
    
    // ctx.layer();
 
    // let v = app.sat.get_points().iter().map(|&x| test(x)).collect::<Vec<_>>();

    // ctx.draw(&Points {
    //     coords: &v,
    //     color: Color::Green
    // });
    ctx.layer();

    let p_module = (p_enu.get_x().powi(2) + p_enu.get_y().powi(2) + p_enu.get_z().powi(2)).sqrt();
    let p_enu_normalized = Vector3::new(p_enu.get_x() / p_module, p_enu.get_y() / p_module, p_enu.get_z() / p_module);

    let p_spheric = Vector3::new(p_enu.get_x().atan2(p_enu.get_y()),  p_enu.get_z().asin(), 0.0);
    
    ctx.print(100.0, 0.0, format!("Elevation: {:.5}", p_spheric.get_y()*(180.0/core::f64::consts::PI)));
    ctx.print(100.0, -10.0, format!("Azimuth: {:.5}", p_spheric.get_x()*(180.0/core::f64::consts::PI)));

    let p = 90.0 - (p_spheric.get_y()*(180.0/core::f64::consts::PI));

    ctx.draw(&Circle {
        x: -p*p_spheric.get_x().cos(),
        y: p*p_spheric.get_x().sin(),
        radius: 5.0,
        color: Color::Blue,
    });
}

fn get_user_location() -> Vector3 // Radius, Longitude and Altitude
{
    Vector3::new(0.0, -58.381555, -34.603599)
}
