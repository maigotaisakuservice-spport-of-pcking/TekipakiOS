use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, Label, Button, Notebook, Entry, CheckButton};

fn main() {
    let app = Application::builder()
        .application_id("com.tekipaki.settings")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tekipaki Settings")
        .default_width(800)
        .default_height(600)
        .build();

    let notebook = Notebook::new();

    // License Page
    let license_box = Box::new(Orientation::Vertical, 10);
    license_box.set_margin_top(20); license_box.set_margin_bottom(20); license_box.set_margin_start(20); license_box.set_margin_end(20);
    license_box.append(&Label::new(Some("License Activation")));
    let key_entry = Entry::builder().placeholder_text("XXXXX-XXXXX-XXXXX-XXXXX-XXXXX").build();
    license_box.append(&key_entry);
    let activate_btn = Button::with_label("Activate OS");
    license_box.append(&activate_btn);
    notebook.append_page(&license_box, Some(&Label::new(Some("Activation"))));

    // Network / RDP Page
    let net_box = Box::new(Orientation::Vertical, 10);
    net_box.set_margin_top(20); net_box.set_margin_bottom(20); net_box.set_margin_start(20); net_box.set_margin_end(20);
    let rdp_check = CheckButton::with_label("Enable Remote Desktop (RDP)");
    net_box.append(&rdp_check);
    let ad_btn = Button::with_label("Join Active Directory Domain");
    net_box.append(&ad_btn);
    notebook.append_page(&net_box, Some(&Label::new(Some("Network & Business"))));

    // Personalization
    let person_box = Box::new(Orientation::Vertical, 10);
    person_box.set_margin_top(20); person_box.set_margin_bottom(20); person_box.set_margin_start(20); person_box.set_margin_end(20);
    person_box.append(&Label::new(Some("Theme: Windows 11 (Standard)")));
    person_box.append(&Button::with_label("Change Wallpaper"));
    notebook.append_page(&person_box, Some(&Label::new(Some("Personalization"))));

    window.set_child(Some(&notebook));
    window.present();
}
