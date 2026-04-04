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
            let list_item = list_item.downcast_ref::<gtk::ListItem>().expect("Not a ListItem");
            let child = list_item.child().expect("No child");
            let label = child.downcast::<Label>().expect("Child is not a Label");
            let item = list_item.item().expect("No item");
            let string_object = item.downcast::<gtk::StringObject>().expect("Item is not a StringObject");
            label.set_text(&string_object.string());
        });

        let selection = gtk::SingleSelection::new(Some(model));
        let list = ListView::new(Some(selection), Some(factory));
        proc_box.append(&list);
        notebook.append_page(&proc_box, Some(&Label::new(Some("Processes"))));

        // Performance Page
        let perf_box = Box::new(Orientation::Vertical, 5);
        perf_box.set_margin_top(10); perf_box.set_margin_bottom(10); perf_box.set_margin_start(10); perf_box.set_margin_end(10);
        let cpu_label = Label::new(Some("CPU: 0% / Loading..."));
        let ram_label = Label::new(Some("RAM: 0 GB / 0 GB"));
        perf_box.append(&cpu_label);
        perf_box.append(&ram_label);

        // Mocking refresh logic
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            cpu_label.set_text("CPU: 14% | 3.2 GHz");
            ram_label.set_text("RAM: 4.8 GB / 16.0 GB");
            glib::ControlFlow::Continue
        });
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
