use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Box, Orientation, Button, ProgressBar};

mod recovery_logic;
mod datalocker;

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
        let update_btn = Button::with_label("Update TRE / TREを更新");
        let datalocker_btn = Button::with_label("DataLocker (Enterprise) / データロッカー");
        let sync_btn = Button::with_label("Cloud Sync (Enterprise) / クラウド同期");

        let status_label = Label::builder().label("Ready / 準備完了").margin_bottom(10).build();

        let progress = ProgressBar::new();
        progress.set_visible(false);

        repair_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.1);
            status_label.set_label("Repairing system files... / システムファイルを修復中...");
            let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());
            std::thread::spawn(move || {
                let res = recovery_logic::check_system_integrity();
                sender.send(res).expect("Could not send through channel");
            });
            receiver.attach(None, glib::clone!(@weak progress, @weak status_label => @default-return glib::ControlFlow::Break, move |res| {
                match res {
                    Ok(_) => {
                        progress.set_fraction(1.0);
                        status_label.set_label("Repair Complete / 修復が完了しました");
                    },
                    Err(e) => {
                        progress.set_fraction(0.0);
                        status_label.set_label(&format!("Error: {}", e));
                    },
                }
                glib::ControlFlow::Break
            }));
        }));

        datalocker_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.5);
            status_label.set_label("Accessing DataLocker... / DataLockerにアクセス中...");

            let dev = recovery_logic::find_datalocker_partition().unwrap_or_else(|| "/dev/sdb1".to_string());
            // Simulation: Attempt to unlock a hypothetical DataLocker partition
            match datalocker::unlock_drive_pqc_gui(&dev) {
                Ok(mount) => {
                    status_label.set_label(&format!("Unlocked: {}", mount));
                    progress.set_fraction(1.0);
                },
                Err(e) => {
                    status_label.set_label(&format!("Error: {}", e));
                    progress.set_fraction(0.0);
                }
            }
        }));

        sync_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.1);
            status_label.set_label("Syncing to Cloud... / クラウドに同期中...");
            let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());
            std::thread::spawn(move || {
                let res = datalocker::sync_to_cloud("/mnt/datalocker", "backup-server:/vault");
                sender.send(res).expect("Could not send through channel");
            });
            receiver.attach(None, glib::clone!(@weak progress, @weak status_label => @default-return glib::ControlFlow::Break, move |res| {
                match res {
                    Ok(_) => {
                        progress.set_fraction(1.0);
                        status_label.set_label("Sync Complete / 同期が完了しました");
                    },
                    Err(e) => {
                        progress.set_fraction(0.0);
                        status_label.set_label(&format!("Error: {}", e));
                    },
                }
                glib::ControlFlow::Break
            }));
        }));

        cloud_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.1);
            status_label.set_label("Reinstalling OS... / OSを再インストール中...");
            let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());
            std::thread::spawn(move || {
                let res = recovery_logic::cloud_reinstall();
                sender.send(res).expect("Could not send through channel");
            });
            receiver.attach(None, glib::clone!(@weak progress, @weak status_label => @default-return glib::ControlFlow::Break, move |res| {
                match res {
                    Ok(_) => {
                        progress.set_fraction(1.0);
                        status_label.set_label("Reinstall Complete / 再インストールが完了しました");
                    },
                    Err(e) => {
                        progress.set_fraction(0.0);
                        status_label.set_label(&format!("Error: {}", e));
                    },
                }
                glib::ControlFlow::Break
            }));
        }));

        reset_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.5);
            status_label.set_label("Resetting PC... / PCを初期化中...");
            let _ = recovery_logic::reset_this_pc();
            progress.set_fraction(1.0);
            status_label.set_label("Reset Complete / 初期化が完了しました");
        }));

        cmd_btn.connect_clicked(|_| {
            recovery_logic::open_command_prompt();
        });

        update_btn.connect_clicked(glib::clone!(@weak progress, @weak status_label => move |_| {
            progress.set_visible(true);
            progress.set_fraction(0.1);
            status_label.set_label("Updating TRE... / TREを更新中...");
            let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());
            std::thread::spawn(move || {
                let res = recovery_logic::update_tre();
                sender.send(res).expect("Could not send through channel");
            });
            receiver.attach(None, glib::clone!(@weak progress, @weak status_label => @default-return glib::ControlFlow::Break, move |res| {
                match res {
                    Ok(_) => {
                        progress.set_fraction(1.0);
                        status_label.set_label("TRE Updated / TREが更新されました");
                    },
                    Err(e) => {
                        progress.set_fraction(0.0);
                        status_label.set_label(&format!("Error: {}", e));
                    },
                }
                glib::ControlFlow::Break
            }));
        }));

        vbox.append(&repair_btn);
        vbox.append(&cloud_btn);
        vbox.append(&reset_btn);
        vbox.append(&cmd_btn);
        vbox.append(&update_btn);
        vbox.append(&datalocker_btn);
        vbox.append(&sync_btn);
        vbox.append(&status_label);
        vbox.append(&progress);

        window.set_child(Some(&vbox));
        window.present();
    });
    app.run();
}
