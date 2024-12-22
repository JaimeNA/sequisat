

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
         .direction(Direction::Horizontal)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(20),
                 Constraint::Percentage(80)
             ].as_ref()
         )
         .split(frame.size());
 
     draw_coords(frame, app, chunks[0]);
 
    //  let map = Canvas::default()
    //      .block(Block::default().title("World").borders(Borders::ALL))
    //      .paint(|ctx| paint_map(ctx, sat))
    //      .marker(symbols::Marker::Braille)
    //      .x_bounds([-180.0, 180.0])
    //      .y_bounds([-90.0, 90.0]);
 
    //  let block = Block::default()
    //       .title("Block 2")
    //       .borders(Borders::ALL);
    
    // frame.render_widget(map, chunks[1]);
}
 
 fn paint_map(ctx: &mut Context, sat: &Satellite)
 {
     
     ctx.draw(&Map {
         color: Color::White,
         resolution: MapResolution::High,
     });
     ctx.layer();    // Go one layer above
                     //
     ctx.draw(&Circle {
         x: (sat.getLongitude()* 180.0/3.14159),
         y: (sat.getLatitude()* 180.0/3.14159),
         radius: 5.0,
         color: Color::Yellow,
     });
 
     ctx.layer();
 
     ctx.draw(&Points {
         coords: sat.get_points(),
         color: Color::Green
     });
 }
 
 
 fn draw_coords(frame: &mut Frame, app: &mut App, area: Rect)
 {
     let coords = Block::default()
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
 
     let altitude = Paragraph::new(text)
         .block(coords)
         .style(Style::default().fg(Color::White));
 
 
    // let latitude = Paragraph::new(format!("Latitude: {}", sat.getLatitude()))
    //     .block(coords)
    //     .style(Style::default().fg(Color::White));
 
     //let longitude = Paragraph::new(format!("Longitude: {}", sat.getLongitude()))
     //    .block(coords)
     //    .style(Style::default().fg(Color::White));
 
     frame.render_widget(altitude, area); 
     //f.render_widget(latitude, chunk);
     //f.render_widget(longitude, chunk);
 }