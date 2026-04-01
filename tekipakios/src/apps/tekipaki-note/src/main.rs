use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, ScrolledWindow, HeaderBar, MenuButton, Box, Orientation};

fn main() {
    let app = Application::builder().application_id("com.tekipaki.note").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tekipaki Notepad")
            .default_width(600)
            .default_height(400)
            .build();

        let text_view = TextView::new();
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&text_view)
            .build();

        window.set_child(Some(&scrolled));
        window.present();
    });
    app.run();
}
