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
    license_box.append(&Label::new(Some("License Activation / ライセンス認証")));
    let key_entry = Entry::builder().placeholder_text("XXXXX-XXXXX-XXXXX-XXXXX-XXXXX").build();
    license_box.append(&key_entry);
    let activate_btn = Button::with_label("Activate OS / ライセンス認証を実行");
    license_box.append(&activate_btn);
    notebook.append_page(&license_box, Some(&Label::new(Some("Activation / 認証"))));

    // Network / RDP Page
    let net_box = Box::new(Orientation::Vertical, 10);
    net_box.set_margin_top(20); net_box.set_margin_bottom(20); net_box.set_margin_start(20); net_box.set_margin_end(20);
    let rdp_check = CheckButton::with_label("Enable Remote Desktop (RDP) / リモートデスクトップを有効にする");
    net_box.append(&rdp_check);

    let domain_entry = Entry::builder().placeholder_text("domain.local").build();
    net_box.append(&Label::new(Some("AD Domain / Active Directory ドメイン:")));
    net_box.append(&domain_entry);

    let ad_btn = Button::with_label("Join Active Directory Domain / ドメインに参加");
    ad_btn.connect_clicked(move |b| {
        let parent = b.root().and_downcast::<gtk::Window>().unwrap();
        let dialog = gtk::MessageDialog::builder()
            .transient_for(&parent)
            .modal(true)
            .message_type(gtk::MessageType::Question)
            .buttons(gtk::ButtonsType::OkCancel)
            .text("Confirm Domain Join / ドメイン参加の確認")
            .secondary_text("Join this PC to the specified Active Directory domain? / このPCをADドメインに参加させますか？")
            .build();

        let domain = domain_entry.text().to_string();
        dialog.connect_response(move |d, res| {
            if res == gtk::ResponseType::Ok {
                println!("Settings: Joining Domain {}...", domain);
                // Simulated command execution
                let status = std::process::Command::new("realm")
                    .args(["join", "--user=admin", &domain])
                    .status();

                if status.map(|s| s.success()).unwrap_or(false) {
                    println!("Settings: Domain Joined Successfully.");
                } else {
                    println!("Settings: Domain Join Simulation Failed (Check realm/samba tools).");
                }
            }
            d.destroy();
        });
        dialog.present();
    });
    net_box.append(&ad_btn);
    notebook.append_page(&net_box, Some(&Label::new(Some("Network & Business / ネットワーク"))));

    // Personalization
    let person_box = Box::new(Orientation::Vertical, 10);
    person_box.set_margin_top(20); person_box.set_margin_bottom(20); person_box.set_margin_start(20); person_box.set_margin_end(20);
    person_box.append(&Label::new(Some("Theme: Tekipaki (Standard) / テーマ: Tekipaki (標準)")));
    person_box.append(&Button::with_label("Change Wallpaper / 壁紙を変更"));
    notebook.append_page(&person_box, Some(&Label::new(Some("Personalization / 個人設定"))));

    window.set_child(Some(&notebook));
    window.present();
}
