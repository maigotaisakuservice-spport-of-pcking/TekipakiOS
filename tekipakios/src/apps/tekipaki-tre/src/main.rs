use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Box, Orientation, Button, ProgressBar};

fn main() {
    let app = Application::builder().application_id("com.tekipaki.tre").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tekipaki Recovery Environment (TRE)")
            .default_width(800)
            .default_height(500)
            .build();

        // Win11 Style Recovery UI
        let vbox = Box::new(Orientation::Vertical, 20);
        vbox.set_margin_top(50); vbox.set_margin_bottom(50); vbox.set_margin_start(50); vbox.set_margin_end(50);

        vbox.append(&Label::builder().label("Choose an option / オプションを選択してください").name("title-1").build());

        let repair_btn = Button::with_label("Auto Repair (Self-Healing) / 自動修復");
        let cloud_btn = Button::with_label("Cloud Reinstall / クラウドから再インストール");
        let reset_btn = Button::with_label("Reset this PC / PCを初期状態に戻す");
        let cmd_btn = Button::with_label("Command Prompt / コマンド プロンプト");

        let progress = ProgressBar::new();
        progress.set_visible(false);

        repair_btn.connect_clicked(glib::clone!(@weak progress => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.0);
            println!("Starting self-healing: fsck /dev/sda2...");
            // Real logic would involve std::process::Command to run fsck or pacman -Syu
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                let f = progress.fraction() + 0.02;
                progress.set_fraction(f);
                if f >= 1.0 { glib::ControlFlow::Break } else { glib::ControlFlow::Continue }
            });
        }));

        cloud_btn.connect_clicked(glib::clone!(@weak progress => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.0);
            println!("Starting cloud reinstall...");
            glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
                let f = progress.fraction() + 0.05;
                progress.set_fraction(f);
                if f >= 1.0 { glib::ControlFlow::Break } else { glib::ControlFlow::Continue }
            });
        }));

        vbox.append(&repair_btn);
        vbox.append(&cloud_btn);
        vbox.append(&reset_btn);
        vbox.append(&cmd_btn);
        vbox.append(&progress);

        window.set_child(Some(&vbox));
        window.present();
    });
    app.run();
}
