use eframe::egui;
use egui::{RichText, FontId, Color32};


struct Spreadsheet {
    len_h: i32,
    len_v: i32,
    top_h: i32,
    top_v: i32,
    database: Vec<i32>,
    err: Vec<bool>
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            len_h: 10,
            len_v: 10,
            top_h: 0,
            top_v: 0,
            database: vec![0; 100],
            err: vec![false; 100]
            
        }
    }
}

impl eframe::App for Spreadsheet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("spreadsheet_grid")
                .striped(true)
                .show(ui, |ui| {
                    for row in 0..10 {
                        for col in 0..10 {
                            ui.label(RichText::new(format!("{}", self.database[(row* (self.len_h) + col) as usize])).font(FontId::proportional(20.0)));
                        }
                        ui.end_row();
                    }
                });

        
        });
    }
}

pub fn run() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Spreadsheet",
        options,
        Box::new(|_cc| {
            Ok(Box::<Spreadsheet>::default())
        }),
    )
}