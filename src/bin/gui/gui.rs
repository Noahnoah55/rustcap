use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.github.Noahnoah55.rcap")
        .build();


    app.connect_activate(|app| {
        let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file("test.png").expect("Image must exist to continue");
        let img = gtk::Image::builder()
            .pixbuf(&pixbuf)
            .build();
        let content = gtk::ScrolledWindow::builder()
            .child(&img)
            .build();
        let win = ApplicationWindow::builder()
            .application(app)
            .build();
        win.add(&content);
        win.show_all();
    });

    app.run();
}