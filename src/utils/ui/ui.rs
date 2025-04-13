use eframe::egui;
use egui::{ Button, Color32, FontId, RichText};

fn shift_char(c:char, i: i32)-> char{
    (c as i8 + i as i8) as u8 as char
}

fn get_label(a : i32)-> String{
    let mut temp = String::new();
    let mut num = a-1;
    if 0 <= num && num <= 25 {
        temp.push(shift_char('A', num));
    } else if 26 <= num && num <= 701 {
        num-=26;
        temp.push(shift_char('A', num/26));
        temp.push(shift_char('A', num%26));
    } else if 702 <= num && num <= 18277 {
        num-=702;
        let c = shift_char('A', num%26);
        num/=26;
        temp.push(shift_char('A', num/26));
        temp.push(shift_char('A', num%26));
        temp.push(c);
    }

    temp
}

pub struct Spreadsheet {
    len_h: i32,
    len_v: i32,
    top_h: i32,
    top_v: i32,
    database: Vec<i32>,
    err: Vec<bool>,
    terminal: String,
    cell_ref: String,
    formula_bar : String
}


impl Spreadsheet {
    pub fn new(len_h: i32, len_v: i32, top_h: i32, top_v: i32, database: Vec<i32>, err: Vec<bool>) -> Self {
        Self {
            len_h,
            len_v,
            top_h,
            top_v,
            database,
            err,
            terminal: String::new(),
            cell_ref: String::new(),
            formula_bar: String::new(),
        }
    }

    pub fn update_database(&mut self, database: Vec<i32>,new_val :i32 ,index: usize) {
        self.database[index] = new_val;
    }
}

impl eframe::App for Spreadsheet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            // Header
            ui.horizontal(|ui| {
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Copy").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Paste").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Describe").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Plot").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("PDF").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Load").font(FontId::proportional(20.0))));
                ui.add_sized([120.0,120.0],egui::Button::new(RichText::new("Save").font(FontId::proportional(20.0))));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let current_date = chrono::Local::now().format("%A, %B %d, %Y").to_string();
                    let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
                    ui.add_sized([310.0, 120.0], egui::Label::new(RichText::new(format!("Rust Spreadsheet Project\n\nDate: {}\nTime: {}",current_date,current_time)).font(FontId::proportional(20.0))));
                });
            });
            ui.add_space(10.0); // Add bottom margin
            ui.horizontal(|ui|{
                ui.add_sized([210.0,30.0], egui::TextEdit::singleline(&mut self.cell_ref).hint_text("Eg. A1").font(FontId::proportional(20.0)));
                ui.add_sized([950.0,30.0], egui::TextEdit::singleline(&mut self.formula_bar).hint_text("Eg. =A2+A3").font(FontId::proportional(20.0)));
                
            });

            ui.add_space(10.0);
            // Main
            egui::Grid::new("spreadsheet_grid")
                .show(ui, |ui| {

                    // Header
                    egui::Frame::new()
                        

                            .show(ui, |ui| {
                                ui.add_sized([70.0, 35.0], 

                                    egui::Label::new(RichText::new(format!("")).font(FontId::proportional(20.0)))
                                );
                                
                            });
                    for col in 0..10 {
                            
                        egui::Frame::new()
                        
                            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                            .show(ui, |ui| {
                                ui.add_sized([100.0, 35.0], 

                                    egui::Label::new(RichText::new(format!("{}", get_label(col+self.top_h))).font(FontId::proportional(20.0)))
                                );
                                
                            });
                        
                    }

                    ui.end_row(); 


                    for row in 0..10 {
                        
                        // Number
                        egui::Frame::new()
                            
                        .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                        .show(ui, |ui| {
                            ui.add_sized([70.0, 45.0], 

                                egui::Label::new(RichText::new(format!("{}",row+self.top_v))
                                    .font(FontId::proportional(20.0))).selectable(false)
                            );
                            
                        });


                        for col in 0..10 {
                            
                            let data = if !(self.err[((self.top_v + row-1) * self.len_h + col+self.top_h) as usize]) {
                                format!("{}", self.database[((self.top_v + row-1) * self.len_h + col+self.top_h) as usize])
                            }else{
                                "ERR".to_string()
                            };

                            egui::Frame::new()
                            
                                .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                                .show(ui, |ui| {
                                    let frame = ui.add_sized([100.0, 45.0], 
                                        
                                        egui::Label::new(RichText::new(data)
                                            .font(FontId::proportional(20.0)))
                                    );
                                    if frame.clicked(){
                                        println!("Clicked on cell {} :{}{}",(self.top_v + row-1) * self.len_h + col+self.top_h, get_label(col+self.top_h), row+self.top_v);
                                    };
                                });
                            
                        }
                        ui.end_row(); // called once per row
                    }
                });

        // Footer
        ui.horizontal(|ui| {
                ui.add(egui::Image::new(egui::include_image!("assets/terminal.jpg")).fit_to_exact_size(egui::vec2(50.0, 50.0)));
                ui.add_sized([700.0,30.0],egui::TextEdit::singleline(&mut self.terminal)
                    .hint_text("Enter command here")
                    .font(FontId::proportional(20.0)));
                ui.add_sized([50.0, 30.0], Button::new(RichText::new("GO").font(FontId::proportional(20.0))));
                if ui.add_sized([50.0, 30.0], Button::new(RichText::new("<").font(FontId::proportional(20.0)))).clicked() {
                    self.top_h-=10;
                };
                if ui.add_sized([50.0, 30.0], Button::new(RichText::new("v").font(FontId::proportional(20.0)))).clicked(){
                    self.top_v+=10;
                };
                ui.add_sized([120.0,30.0], egui::Label::new(RichText::new("00:00").font(FontId::proportional(20.0))));
                if ui.add_sized([50.0, 30.0], Button::new(RichText::new("^").font(FontId::proportional(20.0)))).clicked(){
                    self.top_v-=10;
                };
                if ui.add_sized([50.0, 30.0], Button::new(RichText::new(">").font(FontId::proportional(20.0)))).clicked(){
                    self.top_h+=10;
                };
                
        }
        );
        
        });


        

        
    }
}
