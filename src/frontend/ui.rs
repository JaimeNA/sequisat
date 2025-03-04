use crate::App;
use crate::PositionVector;
use crate::Satellite;
use crate::frontend::app::{
    MessageType,
    Message
};

use ratatui::{
    style::{Style, Color, Modifier},
    widgets::{Borders, Block, Paragraph, Tabs, Clear, List},
    widgets::canvas::{Canvas, Points, Circle, Line, MapResolution, Map, Context},
    prelude::{Constraint, Rect, Direction, Layout, Stylize},
    text::Span,
    symbols,
    text,
    Frame
};

const USAGE: &str = "c - Set user Coordinates | Enter - Clear popups | q - Quit";

const POPUP_WIDTH: u16 = 55;
const POPUP_HEIGHT: u16 = 3;

const DARK_RED: Color = Color::Rgb(150, 24, 16);
const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
const AMBER: Color = Color::Rgb(255, 191, 0);
const GRAY: Color = Color::Rgb(50, 50, 50);
const LIGHT_GRAY: Color = Color::Rgb(150, 150, 150);
const WHITE: Color = Color::Rgb(238, 238, 238); // not really white, often #eeeeee

pub fn draw(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let [title_bar, tab, bottom_bar] = vertical.areas(frame.area()); // TODO: Do something with
                                                                     // bottom_bar
 
    draw_title_bar(frame, app, title_bar);

    // Don't draw anything if there is no satellite
    if let Some(sat) = app.get_sat() { 
        // Draw the selected tab
        match app.tabs.index {
            0 => draw_map_tab(frame, sat, app, tab),
            1 => draw_azimuth_tab(frame, sat, app, tab),
            2 => draw_about_tab(frame, sat, app, tab),
            _ => {}
        };
    }

    if app.input_mode {

        // The middle of the frame
        let x = (frame.area().width - POPUP_WIDTH) / 2;
        let y = (frame.area().height - POPUP_HEIGHT) / 2;

        let area = Rect::new(x, y, POPUP_WIDTH, POPUP_HEIGHT).clamp(frame.area()); // Clamps rect inside the frame

        let position_data = Block::default()
        .title("Set user coordinates: [lat(deg)] [lon(deg)] [alt(km)]")
        .borders(Borders::ALL);
    
        let data = Paragraph::new(app.buffer.clone())
            .block(position_data)
            .style(Style::default().fg(Color::White));

        // Clear area before drawing
        frame.render_widget(Clear, area);
        frame.render_widget(data, area);   
    }
    show_messages(frame, app);
}

fn draw_title_bar(frame: &mut Frame, app: &App, area: Rect)
{
    // Divide title bar
    let layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(43)]);
    let [title_area, tabs_area] = layout.areas(area);

    let title = Span::styled(app.title, Style::new()
        .fg(WHITE)
        .add_modifier(Modifier::BOLD));

    // Get tabs titles and join them on a Span for display, TODO: make it more idiomatic
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

/*
 * Main tabs - Display the data in two different projections and the information used for said
 * projections. 
*/

fn draw_map_tab(frame: &mut Frame, sat: &Satellite, app: &App, area: Rect)
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
         .paint(|ctx| paint_map(ctx, sat, app))
         .marker(symbols::Marker::Braille)
         .x_bounds([-180.0, 180.0])
         .y_bounds([-90.0, 90.0]);

    frame.render_widget(map, chunks[0]);

    // Make smaller chunks for diplay data
    let chunklin = Layout::default()
         .direction(Direction::Vertical)
         .constraints(
             [
                 Constraint::Percentage(15),
                 Constraint::Percentage(85)
             ].as_ref()
         )
         .split(chunks[1]);
    draw_sat_coords(frame, sat, chunklin[0]);
    //draw_user_coords(frame, app, chunklin[1]);
    draw_tle_options(frame, app, chunklin[1]);
}

fn draw_azimuth_tab(frame: &mut Frame, sat: &Satellite, app: &App, area: Rect)
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
        .block(Block::default().title("Azimuth").borders(Borders::ALL))
        .paint(|ctx| paint_azimuth(ctx, sat, app))
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-180.0, 180.0]);

    frame.render_widget(map, chunks[0]);    

    draw_stereographic_coords(frame, sat, app, chunks[1]);   
}


fn draw_about_tab(frame: &mut Frame, sat: &Satellite, app: &App, area: Rect)
{
    let chunks = Layout::default()
         .direction(Direction::Vertical)
         .constraints(
             [
                 Constraint::Percentage(85),
                 Constraint::Percentage(5)
             ].as_ref()
         )
         .split(area);
 
    let chunklin = Layout::default()
         .direction(Direction::Horizontal)
         .constraints(
             [
                 Constraint::Percentage(50),
                 Constraint::Percentage(50)
             ].as_ref()
         )
         .split(chunks[0]);

    draw_tle_data(frame, sat, chunklin[0]);
    draw_user_coords(frame, app, chunklin[1]);

    let text = vec![
        text::Line::from(vec![
            Span::styled("Usage: ", Style::default().fg(Color::Green)),
            Span::styled(USAGE, Style::default().fg(Color::Gray)),
        ]),
        text::Line::from(vec![
            Span::styled("By Jaime Nazar Anchorena - 2025", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let usage = Paragraph::new(text)
        .centered();

    frame.render_widget(usage, chunks[1]);
}

// Blocks

fn paint_map(ctx: &mut Context, sat: &Satellite, app: &App)
{
     
    ctx.draw(&Map {
        color: Color::White,
        resolution: MapResolution::High,
    });

    ctx.layer();    
     
    ctx.draw(&Circle {
        x: app.get_usr_geodetic().get_y() * (180.0/core::f64::consts::PI),
        y: app.get_usr_geodetic().get_x() * (180.0/core::f64::consts::PI),
        radius: 1.0,
        color: Color::Red,
    });
 
    ctx.layer();
 
    ctx.draw(&Points {
        coords: sat.get_points(),
        color: Color::Green
    });

    ctx.layer();    // Go one layer above
                     //
    ctx.draw(&Circle {
        x: (sat.get_geodetic_position().get_y()* 180.0/3.14159),
        y: (sat.get_geodetic_position().get_x()* 180.0/3.14159),
        radius: 5.0,
        color: Color::Yellow,
    });
}

fn draw_stereographic_coords(frame: &mut Frame, sat: &Satellite, app: &App, area: Rect)
{
    let position_data = Block::default()
    .title("Stereographic Coordinates")
    .borders(Borders::ALL);

    // Get Elevation and Azimuth
    let el_az = get_azimuth_and_elevation(&app.get_usr_geodetic(), &sat.get_geodetic_position());

    let text = vec![
        text::Line::from(vec![
            Span::from("Azimuth: "),
            Span::styled(format!("{:.5} deg", (el_az.get_x() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
        ]),
        text::Line::from(vec![
            Span::from("Elevation: "),
            Span::styled(format!("{:.5} deg",(el_az.get_y() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
        ])
    ];

    let data = Paragraph::new(text)
        .block(position_data)
        .style(Style::default().fg(Color::White));

    frame.render_widget(data, area);   
}

fn draw_user_coords(frame: &mut Frame, app: &App, area: Rect)
{
    let position_data = Block::default()
    .title("User Coordinates")
    .borders(Borders::ALL);

    let text = vec![
        text::Line::from(vec![
            Span::from("Latitude: "),
            Span::styled(format!("{:.5} deg", (app.get_usr_geodetic().get_x() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
        ]),
        text::Line::from(vec![
            Span::from("Longitude: "),
            Span::styled(format!("{:.5} deg",(app.get_usr_geodetic().get_y() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("Altitude: "),
            Span::styled(format!("{:.5} km", app.get_usr_geodetic().get_z().to_string()), Style::default().fg(Color::Red)),
        ])
    ];

    let data = Paragraph::new(text)
        .block(position_data)
        .style(Style::default().fg(Color::White));

    frame.render_widget(data, area);   
}

fn draw_sat_coords(frame: &mut Frame, sat: &Satellite, area: Rect) // Repeated code, fix later
{
    let position_data = Block::default()
        .title("Satellite Coordinates")
        .borders(Borders::ALL);
 
    let text = vec![
        text::Line::from(vec![
            Span::from("Altitude: "),
            Span::styled(format!("{:.5} km", sat.get_geodetic_position().get_z().to_string()), Style::default().fg(Color::Red)),
        ]),
        text::Line::from(vec![
            Span::from("Longitude: "),
            Span::styled(format!("{:.5} deg", (sat.get_geodetic_position().get_y() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("Latitude: "),
            Span::styled(format!("{:.5} deg", (sat.get_geodetic_position().get_x() * (180.0/core::f64::consts::PI)).to_string()), Style::default().fg(Color::Blue)),
        ])
    ];
 
    let data = Paragraph::new(text)
        .block(position_data)
        .style(Style::default().fg(Color::White));
 
    frame.render_widget(data, area); 
}

fn draw_tle_data(frame: &mut Frame, sat: &Satellite, area: Rect)
{
    let tle_data = Block::default()
        .title("TLE Data")
        .borders(Borders::ALL);
 
    let text = vec![
        text::Line::from(vec![
            Span::from("Satellite Catalog Number: "),
            Span::styled(sat.get_tle().get_catalog_number().to_string(), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Classification: "),
            Span::styled(sat.get_tle().get_classification(), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Launch Year: "),
            Span::styled(sat.get_tle().get_launch_year().to_string(), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Launch Piece: "),
            Span::styled(sat.get_tle().get_launch_piece().to_string(), Style::default().fg(Color::Yellow)),
        ]),
        // TODO: change color of smth to mark them as different
        text::Line::from(vec![
            Span::from("Epoch Year: "),
            Span::styled(sat.get_tle().get_epoch_year().to_string(), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Epoch Day of Year: "),
            Span::styled(format!("{:.5}", sat.get_tle().get_epoch_day().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Ballistic Coefficient: "),
            Span::styled(format!("{:.10}", sat.get_tle().get_ballistic_coefficient()).to_string(), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Drag Term: "),
            Span::styled(format!("{:.10}", sat.get_tle().get_drag_term().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Inclination(rads): "),
            Span::styled(format!("{:.5}", sat.get_tle().get_inclination().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Right Ascension(rads): "),
            Span::styled(format!("{:.5}", sat.get_tle().get_right_ascension().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Eccentricity: "),
            Span::styled(format!("{:.5}", sat.get_tle().get_eccetricity().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Argument of Perigee(rads): "),
            Span::styled(format!("{:.5}", sat.get_tle().get_argument_of_perigee().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Mean Anomaly(rads): "),
            Span::styled(format!("{:.5}", sat.get_tle().get_mean_anomaly().to_string()), Style::default().fg(Color::Yellow)),
        ]),
        text::Line::from(vec![
            Span::from("Mean Motion(rads/min): "),
            Span::styled(format!("{:.5}", sat.get_tle().get_mean_motion().to_string()), Style::default().fg(Color::Yellow)),
        ]),
    ];
 
    let data = Paragraph::new(text)
        .block(tle_data)
        .style(Style::default().fg(Color::White));
 
    frame.render_widget(data, area); 
}

fn paint_azimuth(ctx: &mut Context, sat: &Satellite, app: &App)
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
    ctx.print(90.0, 0.0, "W");
    ctx.print(-90.0, 0.0, "E");


    // ctx.layer();
 
    // let v = app.get_sat().get_points().iter().map(|&x| test(x)).collect::<Vec<_>>();

    // ctx.draw(&Points {
    //     coords: &v,
    //     color: Color::Green
    // });
    ctx.layer();

    // Get Elevation and Azimuth
    let el_az = get_azimuth_and_elevation(&app.get_usr_geodetic(), &sat.get_geodetic_position());
    
    let p = 90.0 - (el_az.get_y()*(180.0/core::f64::consts::PI));

    ctx.draw(&Circle {
        x: -p*el_az.get_x().sin(),
        y: p*el_az.get_x().cos(),
        radius: 5.0,
        color: Color::Blue,
    });
}

fn draw_tle_options(frame: &mut Frame, app: &App, area: Rect)
{
    let title = "TLE Options";
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);


    let list = List::new(app.tle_list.items.clone())
        .block(block)
        .style(Style::new().white())
        .highlight_style(Style::new()
            .fg(LIGHT_GRAY)
            .bg(DARK_BLUE)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);
    
    frame.render_widget(Clear, area);  
    frame.render_stateful_widget(list, area, &mut app.tle_list.state.borrow_mut()); 
    
    
}

fn show_messages(frame: &mut Frame, app: &App){
    let lst_msg = app.get_messages().last();
    if let Some(msg) = lst_msg {
        draw_popup_message(frame, msg);
    }
    
}

fn draw_popup_message(frame: &mut Frame, msg: &Message) {

    // Popups appear on the top-left corner
    let area = Rect::new(0, 0, POPUP_WIDTH, POPUP_HEIGHT).clamp(frame.area());
    
    let color = match msg.get_type() {
        MessageType::Error => DARK_RED,
        MessageType::Warning => AMBER,
        MessageType::Info => GRAY,
    };

    let title = format!("Message: {}", msg.get_type().name());
    let block = Block::default()
    .title(title)
    .borders(Borders::ALL)
    .bg(color);

    let data = Paragraph::new(msg.get_message().clone())
    .block(block)
    .style(Style::default().fg(Color::White));

    // Clear area before displaying
    frame.render_widget(Clear, area);  
    frame.render_widget(data, area);   

}

fn get_azimuth_and_elevation(usr_geodetic: &PositionVector, sat_geodetic: &PositionVector) -> PositionVector {
    // Get Elevation and Azimuth
    let usr_ecef = usr_geodetic.geodetic_to_ecef();
    let sat_ecef = sat_geodetic.geodetic_to_ecef();
    
    let p_enu = PositionVector::ecef_to_enu(&usr_ecef, &sat_ecef);
    
    p_enu.enu_to_azimuth_and_elevation()
}
