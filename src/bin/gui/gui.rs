use gtk::gdk_pixbuf::{Pixbuf, InterpType};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, IconSize};
use std::sync::{Arc, Mutex};

fn main() {
    let imgpath = "test.png";

    let app = Application::builder()
        .application_id("com.github.Noahnoah55.rcap")
        .build();

    app.connect_activate(move |app| {
        let zoom_level = Arc::new(Mutex::new(1.0));
        let pixbuf = Pixbuf::from_file(imgpath)
            .expect("Image must exist to continue");
        let img = gtk::Image::builder()
            .pixbuf(&pixbuf)
            .expand(true)
            .build();
        let content = gtk::ScrolledWindow::builder()
            .child(&img)
            .build();

        let zoom_in = gtk::Button::from_icon_name(Some("zoom-in"), IconSize::Button);
        let zoom_out = gtk::Button::from_icon_name(Some("zoom-out"), IconSize::Button);
        let menubar = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .child(&zoom_in)
            .child(&zoom_out)
            .halign(gtk::Align::End)
            .build();

        // Tears start here
        // Don't do UI in rust, kids
        let img2 = img.clone();
        let mut1 = zoom_level.clone();
        zoom_in.connect_clicked(move |_| {
            let mut zlock = mut1.lock().unwrap();
            *zlock = *zlock * 1.2;
            let pbuf = zoom(imgpath, *zlock);
            img2.set_pixbuf(Some(&pbuf));
            ()
        });

        let img3 = img.clone();
        let mut2 = zoom_level.clone();
        zoom_out.connect_clicked(move |_| {
            let mut zlock = mut2.lock().unwrap();
            *zlock = *zlock * 0.8;
            let pbuf = zoom(imgpath, *zlock);
            img3.set_pixbuf(Some(&pbuf));
            ()
        });

        let holder = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .child(&content)
            .child(&menubar)
            .build();
        let win = ApplicationWindow::builder()
            .application(app)
            .events(gtk::gdk::EventMask::KEY_PRESS_MASK)
            .build();
        win.add(&holder);
        win.show_all();
    });

    app.run();
}

fn zoom(path: &str, scale: f64) -> Pixbuf {
    let img = Pixbuf::from_file(path).unwrap();
    let new_height = img.height() as f64 * scale;
    let new_width = img.width() as f64 * scale;
    img.scale_simple(new_width.ceil() as i32, new_height.ceil() as i32, InterpType::Nearest)
        .unwrap()
}