use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListView, Label, Box, Orientation, Notebook, StringList};

fn main() {
    let app = Application::builder().application_id("com.tekipaki.taskmgr").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Task Manager")
            .default_width(700)
            .default_height(500)
            .build();

        let notebook = Notebook::new();

        // Processes Page
        let proc_box = Box::new(Orientation::Vertical, 5);
        proc_box.set_margin_top(10); proc_box.set_margin_bottom(10); proc_box.set_margin_start(10); proc_box.set_margin_end(10);
        proc_box.append(&Label::new(Some("Processes (Mock) / プロセス (モック)")));

        let model = StringList::new(&[
            "System (システム)", "tekipaki-guardd (認証エンジン)", "Dolphin (エクスプローラー)", "Firefox (ブラウザ)", "Settings (設定)", "Task Manager (タスクマネージャー)"
        ]);
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(None);
            list_item.set_child(Some(&label));
        });
        factory.connect_bind(|_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<Label>().unwrap();
            let string_object = list_item.item().unwrap().downcast::<gtk::StringObject>().unwrap();
            label.set_text(&string_object.string());
        });

        let selection = gtk::SingleSelection::new(Some(model));
        let list = ListView::new(Some(selection), Some(factory));
        proc_box.append(&list);
        notebook.append_page(&proc_box, Some(&Label::new(Some("Processes"))));

        // Performance Page
        let perf_box = Box::new(Orientation::Vertical, 5);
        perf_box.set_margin_top(10); perf_box.set_margin_bottom(10); perf_box.set_margin_start(10); perf_box.set_margin_end(10);
        perf_box.append(&Label::new(Some("CPU: 12% | RAM: 3.2 GB / 16.0 GB")));
        notebook.append_page(&perf_box, Some(&Label::new(Some("Performance"))));

        // Startup
        let start_box = Box::new(Orientation::Vertical, 5);
        start_box.append(&Label::new(Some("Startup Apps")));
        notebook.append_page(&start_box, Some(&Label::new(Some("Startup"))));

        window.set_child(Some(&notebook));
        window.present();
    });
    app.run();
}
