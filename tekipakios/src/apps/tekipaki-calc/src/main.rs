use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Grid, Button, Orientation, Box};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = Application::builder().application_id("com.tekipaki.calc").build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tekipaki Calculator")
        .default_width(280)
        .default_height(400)
        .build();

    let vbox = Box::new(Orientation::Vertical, 5);
    let display = Rc::new(Label::builder().label("0").halign(gtk::Align::End).margin_all(10).build());
    vbox.append(display.as_ref());

    let current_val = Rc::new(RefCell::new(String::new()));

    let grid = Grid::builder().column_spacing(2).row_spacing(2).build();
    let buttons = [
        "7", "8", "9", "/",
        "4", "5", "6", "*",
        "1", "2", "3", "-",
        "0", "C", "=", "+"
    ];

    for (i, &label) in buttons.iter().enumerate() {
        let btn = Button::with_label(label);
        let display_cloned = display.clone();
        let val_cloned = current_val.clone();
        btn.connect_clicked(move |b| {
            let text = b.label().unwrap();
            let mut val = val_cloned.borrow_mut();
            if text == "C" {
                val.clear();
                display_cloned.set_label("0");
            } else if text == "=" {
                // Simplified eval simulation
                display_cloned.set_label("Result");
                val.clear();
            } else {
                val.push_str(&text);
                display_cloned.set_label(&val);
            }
        });
        grid.attach(&btn, (i % 4) as i32, (i / 4) as i32, 1, 1);
    }

    vbox.append(&grid);
    window.set_child(Some(&vbox));
    window.present();
}
