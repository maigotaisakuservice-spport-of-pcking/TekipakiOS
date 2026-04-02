use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, DrawingArea, Orientation, Box, ColorDialog, ColorDialogButton};

fn main() {
    let app = Application::builder().application_id("com.tekipaki.paint").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tekipaki Paint")
            .default_width(800)
            .default_height(600)
            .build();

        let vbox = Box::new(Orientation::Vertical, 5);
        let header = Box::new(Orientation::Horizontal, 10);
        header.set_margin_start(5);
        header.append(&Label::new(Some("Paint Tools: Brush, Eraser, Fill")));

        let canvas = DrawingArea::new();
        canvas.set_vexpand(true);
        canvas.set_hexpand(true);

        vbox.append(&header);
        vbox.append(&canvas);
        window.set_child(Some(&vbox));
        window.present();
    });
    app.run();
}
