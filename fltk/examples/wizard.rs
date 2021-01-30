use fltk::*;

pub fn show_wizard() {
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Wizard");
    let mut wiz = group::Wizard::default().with_size(400, 300);
    let grp1 = group::Group::default().size_of(&wiz);
    let mut but = button::Button::default()
        .with_size(80, 40)
        .center_of(&wiz)
        .with_label("next");
    grp1.end();
    let grp2 = group::Group::default().size_of(&wiz);
    let mut but2 = button::Button::default()
        .with_size(80, 40)
        .center_of(&wiz)
        .with_label("previous");
    grp2.end();
    wiz.end();
    win.end();
    win.show();
    but.set_callback({
        let mut wiz_c = wiz.clone();
        move || wiz_c.next()
    });
    but2.set_callback(move || wiz.prev());
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let _but = {
        let mut b = button::Button::default()
            .with_size(160, 40)
            .with_label("Show wizard")
            .center_of(&win);
        b.set_callback(|| show_wizard());
        b
    };
    win.end();
    win.show();
    app.run().unwrap();
}
