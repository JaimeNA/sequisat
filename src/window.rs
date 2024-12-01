use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

pub fn run() {
    env_logger::init();

    let mut window_attributes = Window::default_attributes()
            .with_title("Winit window")
            .with_transparent(true)
            .with_window_icon(Some(self.icon.clone()));

    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let window = Some(event_loop.create_window(window_attributes).unwrap());


}
