use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Box, Orientation, Button};

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

        vbox.append(&Label::builder().label("Choose an option / オプションを選択してください").css_name("title-1").build());

        let repair_btn = Button::with_label("Auto Repair (Self-Healing) / 自動修復");
        let cloud_btn = Button::with_label("Cloud Reinstall / クラウドから再インストール");
        let reset_btn = Button::with_label("Reset this PC / PCを初期状態に戻す");
        let cmd_btn = Button::with_label("Command Prompt / コマンド プロンプト");

        vbox.append(&repair_btn);
        vbox.append(&cloud_btn);
        vbox.append(&reset_btn);
        vbox.append(&cmd_btn);

        window.set_child(Some(&vbox));
        window.present();
    });
    app.run();
}
