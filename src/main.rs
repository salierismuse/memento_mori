use eframe::{egui, NativeOptions};
use chrono::prelude::*;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use chrono::Weekday;

struct Day {
    date: i32,
    month: i32,
    year: i32,
}

struct User {
    name: String,
    bday: Day,
    dday: Day,
}

fn clock(d_year: i32) {
    let now = Local::now();
    let year = now.year();
    let _year_left = d_year - year;
    // unused for now
}

////////////////////////////////////////////////////////////////////////////////

struct MyApp {
    weeks_left: Arc<Mutex<u32>>,
}

fn weeks_remaining(death_date: NaiveDate) -> u32 {
    let today = Local::now().date_naive();
    let duration = death_date - today;
    duration.num_weeks().max(0) as u32
}

impl MyApp {
    fn new() -> Self {
        let death_date = NaiveDate::from_ymd_opt(2085, 4, 7).unwrap();
        let initial_weeks = weeks_remaining(death_date);
        let weeks_left = Arc::new(Mutex::new(initial_weeks));
        let weeks_left_clone = Arc::clone(&weeks_left);

        thread::spawn(move || loop {
            {
                let mut data = weeks_remaining(death_date);
                let today = Local::now().date_naive();
                
            }
            thread::sleep(Duration::from_secs(1));
        });

        Self { weeks_left }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let weeks = *self.weeks_left.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("You could leave life right now.");
                ui.label(format!("Weeks left: {}", weeks));
            });
        });

        ctx.request_repaint(); // keeps UI live
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Memento Mori Clock",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}
