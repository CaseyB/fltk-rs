use fltk::{app::*, button::*, frame::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().size_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.end();
    wind.show();

    but.set_callback(move || frame.set_label("Hello world"));

    app.run().unwrap();
}
