use eframe::egui;
use egui::{Button, Color32, FontId, RichText};
use crate::utils;


pub struct Spreadsheet {
    len_h: i32,
    len_v: i32,
    top_h: i32,
    top_v: i32,
    database: Vec<i32>,
    err: Vec<bool>,
    terminal: String,
    cell_ref: (String,bool,bool),
    selected_cell: Option<i32>,
    hovered_cell: Option<i32>,
    opers : Vec<crate::OPS>,
    indegree : Vec<i32>,
    sensi : Vec<Vec<i32>>,
    temp_txt : (String,bool),
    formula : Vec<String>

}


impl Spreadsheet {
    pub fn new(len_h: i32, len_v: i32, top_h: i32, top_v: i32, database: Vec<i32>, err: Vec<bool>,opers: Vec<crate::OPS>,indegree: Vec<i32>,sensi: Vec<Vec<i32>>) -> Self {
        Self {
            len_h,
            len_v,
            top_h,
            top_v,
            database,
            err,
            terminal: String::new(),
            cell_ref: (String::new(),false,false),
            selected_cell: None,
            hovered_cell: None,
            opers,
            indegree,
            sensi,
            temp_txt: (String::new(),false),
            formula: vec![String::new(); (len_h*len_v + 1) as usize],
        }
    }

}

impl eframe::App for Spreadsheet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)){
                self.top_v-=1;
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)){
                self.top_h-=1;
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)){
                self.top_h+=1;
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)){
                self.top_v+=1;
            }

            ui.add_space(10.0);
            // Header
            ui.horizontal(|ui| {
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/copy.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/paste.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/describe.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/plot.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/pdf.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/folder.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/save.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let current_date = chrono::Local::now().format("%A, %B %d, %Y").to_string();
                    let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
                    ui.add_sized([310.0, 80.0], egui::Label::new(RichText::new(format!("Rust Spreadsheet Project\n\nDate: {}\nTime: {}",current_date,current_time)).font(FontId::proportional(20.0))));
            });});
            ui.horizontal(|ui| {
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Copy").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Paste").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Describe").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Plot").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("PDF").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Load").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Save").font(FontId::proportional(15.0))));
                });

            ui.add_space(10.0); // Add bottom margin
            ui.horizontal(|ui|{

                if self.cell_ref.1 {

                let cell = ui.add_sized([210.0,30.0], egui::TextEdit::singleline(&mut self.cell_ref.0).hint_text("Eg. A1").font(FontId::proportional(20.0)));
                
                if self.cell_ref.2 {
                    cell.request_focus();
                    self.cell_ref.2 = true;
                }

                if cell.gained_focus() {
                    if self.selected_cell != None {
                        self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.selected_cell.unwrap()%self.len_h), self.selected_cell.unwrap()/self.len_h + 1);
                    }else{
                        
                        self.cell_ref.0 = String::new();
                        
                    }
                    
                    self.cell_ref.1 = true;
                }

                if cell.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let temp = format!("scroll_to({})", self.cell_ref.0);
                    let out = utils::input::input(&temp, self.len_h, self.len_v);
                    let status = out[4].clone();
                    if status == "ok" {
                        if out[1] == "SRL"{   
                            let t = crate::cell_to_ind(out[0].as_str(), self.len_h);
                            let mut x1 = t%self.len_h; if x1==0{x1=self.len_h;}
                            let y1 = t/self.len_h + ((x1!=self.len_h) as i32);

                            if x1 < self.top_h || x1 >= self.top_h + 10 || y1 < self.top_v || y1 >= self.top_v + 10 {
                                let mut shift_h = 0;
                                let mut shift_v = 0;

                                if x1 < self.top_h {
                                    shift_h = x1 - self.top_h;
                                } else if x1 >= self.top_h + 10 {
                                    shift_h = x1 - (self.top_h + 9);
                                }

                                if y1 < self.top_v {
                                    shift_v = y1 - self.top_v;
                                } else if y1 >= self.top_v + 10 {
                                    shift_v = y1 - (self.top_v + 9);
                                }

                                self.top_h += shift_h;
                                self.top_v += shift_v;
                            }
                            self.selected_cell = Some(t);
                            self.temp_txt.1 = true;
                        }
                    }
                    self.cell_ref.1 = false;
                };
            }else{

                
                
                if self.selected_cell != None {
                    self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.selected_cell.unwrap()%self.len_h), self.selected_cell.unwrap()/self.len_h + 1);
                }else{
                    if self.hovered_cell!=None{
                        self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.hovered_cell.unwrap()%self.len_h), self.hovered_cell.unwrap()/self.len_h + 1);
                    }else{
                    self.cell_ref.0 = String::new();
                    }
                }
                
                
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                    .show(ui, |ui| {
                        let cell = ui.add_sized([210.0,30.0], egui::Label::new(RichText::new(format!("{}",self.cell_ref.0)).font(FontId::proportional(20.0))));
                        
                        
                        if cell.clicked() {
                                            self.cell_ref.1 = true;
                                        }


                    });

                
            }

            
            


            egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
            .show(ui, |ui| {
                ui.add_sized([950.0,30.0], egui::Label::new(RichText::new(format!("{}",self.temp_txt.0)).font(FontId::proportional(20.0))));
                });
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

                                    egui::Label::new(RichText::new(format!("{}", utils::display::get_label(col+self.top_h))).font(FontId::proportional(20.0)))
                                );
                                
                            });
                        
                    }

                    ui.end_row(); 

                    self.hovered_cell = None;
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
                            let ind = (self.top_v + row-1) * self.len_h + col+self.top_h;
                            egui::Frame::new()
                            
                                .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                                .show(ui, |ui| {
                                    if self.selected_cell == None || (self.selected_cell.unwrap() != ind) {
                                    let frame = ui.add_sized([100.0, 45.0], 
                                        
                                        egui::Label::new(RichText::new(data)
                                            .font(FontId::proportional(20.0)))
                                    );
                                    if frame.clicked(){
                                        
                                        self.selected_cell = Some(ind);
                                        // println!("{:?}",self.selected_cell);
                                        
                                        self.temp_txt.1 = true;
                                    };

                                    if frame.hovered(){
                                        self.hovered_cell = Some(ind);
                                        
                                    }
                                }else{
                                    let ind = self.selected_cell.unwrap();
                                    
                                    let field  = ui.add_sized([100.0, 45.0], 
                                        
                                        egui::TextEdit::singleline(&mut self.temp_txt.0)
                                            .font(FontId::proportional(20.0)).vertical_align(egui::Align::Center).horizontal_align(egui::Align::Center)
                                    );

                                    if self.temp_txt.1{
                                        field.request_focus();
                                        
                                        self.temp_txt.1 = false;
                                    }

                                    if field.gained_focus(){
                                        self.temp_txt.0 = format!("{}",self.formula[ind as usize]);
                                        
                                    }

                                    if field.lost_focus() {
                                        if self.temp_txt.0.starts_with('=') {
                                            self.temp_txt.0.remove(0);
                                        }
                                        self.formula[ind as usize] = self.temp_txt.0.clone();
                                        self.temp_txt.0 = format!("{}{}={}",utils::display::get_label(col+self.top_h), row+self.top_v,self.temp_txt.0);
                                        
                                        self.selected_cell = None;
                                        let out = utils::input::input(&self.temp_txt.0, self.len_h, self.len_v);
                                        let status = out[4].clone();
                                        if status == "ok" {
                                            if out[1] != "SRL"{   
                                                let suc = crate::cell_update(&out, &mut self.database, &mut self.sensi, &mut self.opers, self.len_h, &mut self.indegree, &mut self.err);
                                                if suc==0{
                                                    // Write code for error here
                                                }
                                            }
                                        }
                                        self.temp_txt.0 = String::new();
                                    }
                                }
                                });
                            
                        }
                        ui.end_row(); // called once per row
                    }
                });

        // Footer
        ui.add_space(10.0);
        ui.horizontal(|ui| {
                ui.add(egui::Image::new(egui::include_image!("assets/terminal.png")).fit_to_exact_size(egui::vec2(50.0, 30.0)));
                let term = ui.add_sized([700.0,30.0],egui::TextEdit::singleline(&mut self.terminal)
                    .hint_text("Enter command here")
                    .font(FontId::proportional(20.0)));
                let go = ui.add_sized([50.0, 30.0], Button::new(RichText::new("GO").font(FontId::proportional(20.0))));
                
                if go.clicked() || (term.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    let mut cell  = String::new();
                    let mut formullaaaa = String::new();
                    if self.terminal.contains('=') {
                        let parts: Vec<&str> = self.terminal.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            cell = parts[0].trim().to_string();
                            formullaaaa = parts[1].trim().to_string();
                        }
                    }
                    self.formula[crate::cell_to_ind(cell.as_str(), self.len_h) as usize] = formullaaaa;
                    let out = utils::input::input(&self.terminal, self.len_h, self.len_v);
                    let status = out[4].clone();
                    if status == "ok" {
                        if out[1] == "SRL"{
                            let t = crate::cell_to_ind(out[0].as_str(), self.len_h);
                            let mut x1 = t%self.len_h; if x1==0{x1=self.len_h;}
                            let y1 = t/self.len_h + ((x1!=self.len_h) as i32);
                            self.top_h = x1; self.top_v = y1;
                                                
                        }
                        else{
                            let suc = crate::cell_update(&out, &mut self.database, &mut self.sensi, &mut self.opers, self.len_h, &mut self.indegree, &mut self.err);
                            if suc==0{
                                // status = "cycle_detected".to_string();
                            }
                        }
                    }
                    self.terminal = String::new();
                    term.request_focus();
                };
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
