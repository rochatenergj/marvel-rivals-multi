use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use std::sync::{Arc, Mutex};
use std::thread;

struct GameFeatures {
    aimbot_enabled: bool,
    speed_hack_enabled: bool,
    infinite_health_enabled: bool,
}

impl GameFeatures {
    fn new() -> Self {
        GameFeatures {
            aimbot_enabled: false,
            speed_hack_enabled: false,
            infinite_health_enabled: false,
        }
    }

    fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    fn toggle_speed_hack(&mut self) {
        self.speed_hack_enabled = !self.speed_hack_enabled;
    }

    fn toggle_infinite_health(&mut self) {
        self.infinite_health_enabled = !self.infinite_health_enabled;
    }
}

fn main() {
    let application = Application::new(Some("com.example.marvel_rivals_multi"), Default::default());
    let features = Arc::new(Mutex::new(GameFeatures::new()));

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Marvel Rivals Multi");
        window.set_default_size(300, 200);

        let label = Label::new(Some("Welcome to Marvel Rivals Multi"));
        let aimbot_button = Button::with_label("Toggle Aimbot");
        let speed_hack_button = Button::with_label("Toggle Speed Hack");
        let health_button = Button::with_label("Toggle Infinite Health");

        let features_clone = Arc::clone(&features);
        aimbot_button.connect_clicked(move |_| {
            let mut features = features_clone.lock().unwrap();
            features.toggle_aimbot();
        });

        let features_clone = Arc::clone(&features);
        speed_hack_button.connect_clicked(move |_| {
            let mut features = features_clone.lock().unwrap();
            features.toggle_speed_hack();
        });

        let features_clone = Arc::clone(&features);
        health_button.connect_clicked(move |_| {
            let mut features = features_clone.lock().unwrap();
            features.toggle_infinite_health();
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&aimbot_button, true, true, 0);
        vbox.pack_start(&speed_hack_button, true, true, 0);
        vbox.pack_start(&health_button, true, true, 0);

        window.add(&vbox);
        window.show_all();
    });

    application.run();
}