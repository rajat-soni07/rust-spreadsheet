use eframe::egui;
use egui::{Button, Color32, FontId, RichText};
use crate::utils;
use crate::utils::ui;
use notify_rust::Notification;

#[derive(serde::Serialize, serde::Deserialize, Debug,PartialEq,Clone)]
enum Save {
    Rsk,
    Csv
}

#[derive(serde::Serialize, serde::Deserialize, Debug,PartialEq,Clone)]
enum Plot {
    Line,
    Scatter
}



#[derive(serde::Serialize, serde::Deserialize, Debug,Clone)]
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
    formula : Vec<String>,

    // Save_dialog
    save_dialog: bool,
    save_path: String,
    save_name: String,
    save_type: Save,
    save_todo: Option<(Save,String)>,

    // Load_dialog
    load_dialog: bool,
    load_path: String,
    load_todo: bool,

    // Plot dialog
    plot_dialog: bool,
    plot_x_axis: String,
    plot_y_axis: String,
    plot_rows: String,
    plot_type: Plot,
    plot_save: String,
    plot_todo: bool,

    // PDF dialog
    pdf_dialog: bool,
    pdf_path: String,
    pdf_todo: bool,

    clipbaord: String,

    // Describe dialog
    describe_dialog: bool,
    describe_range: String,
    describe_data: [f64; 8],

    // About dialog
    about_dialog: bool,
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

            // Save_dialog
            save_dialog : false,
            save_path : String::new(),
            save_name : String::new(),
            save_type : Save::Rsk,
            save_todo : None,

            // Load_dialog

            load_dialog : false,
            load_path : String::new(),
            load_todo : false,

            // Plot dialog
            plot_dialog: false,
            plot_x_axis: String::new(),
            plot_y_axis: String::new(),
            plot_rows: String::new(),
            plot_type: Plot::Line,
            plot_save: String::new(),
            plot_todo: false,

            // PDF dialog

            pdf_dialog: false,
            pdf_path: String::new(),
            pdf_todo: false,

            clipbaord: String::new(),

            // Describe dialog
            describe_dialog: false,
            describe_range: String::new(),
            describe_data: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            
            // About dialog
            about_dialog: false,
        }
    }

}

impl eframe::App for Spreadsheet {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Save dialog
        egui::Window::new("Save Spreadsheet")
        .open(&mut self.save_dialog)
        .order(egui::Order::Foreground)
        .fixed_size(egui::vec2(800.0, 500.0))
        
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.add_sized([500.0,30.0],egui::TextEdit::singleline(&mut self.save_name).hint_text("Enter file name").font(FontId::proportional(20.0)));
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.add_sized([400.0,30.0],egui::TextEdit::singleline(&mut self.save_path).hint_text("Enter folder path").font(FontId::proportional(20.0)));
                // ui.text_edit_singleline(&mut self.save_path);
                if ui.add_sized([90.0,30.0],Button::new(RichText::new("Browse").font(FontId::proportional(20.0)))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.save_path = path.display().to_string();
                    }
                };
                
            });
            ui.add_space(10.0);
            
            
            ui.horizontal(|ui| {
                ui.label("\t\t\t\t\t\t\t");

                if ui.add(egui::RadioButton::new(self.save_type==Save::Rsk, RichText::new("RSK\t\t\t\t\t\t\t\t").font(FontId::proportional(20.0)))).on_hover_text("Save to a custom file extension that saves the state of program when you next load it").clicked() {
                    self.save_type = Save::Rsk;
                }
                if ui.add(egui::RadioButton::new(self.save_type==Save::Csv, RichText::new("CSV").font(FontId::proportional(20.0)))).on_hover_text("Save all visible values to a CSV but all the formula's are lost").clicked() {
                    self.save_type = Save::Csv;
                }

            });
            ui.horizontal(|ui|{
                ui.label("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t");

                if ui.add_sized([100.0,30.0], Button::new(RichText::new("Save").font(FontId::proportional(20.0)))).clicked() {
                    if self.save_type == Save::Rsk {
                        let path = format!("{}/{}.rsk", self.save_path,self.save_name);
                        self.save_todo = Some((self.save_type.clone(),path));
                        
                    } else if self.save_type == Save::Csv {
                        let path = format!("{}/{}.csv", self.save_path,self.save_name);
                        self.save_todo = Some((self.save_type.clone(),path));
                        
    
                    }
                    
                }

            });
        });

        if self.save_todo.is_some(){
            println!("{:?}",self.save_todo);
            let (save_type, path) = self.save_todo.clone().unwrap();
            self.save_todo = None;
            self.save_dialog = false;
            match save_type {
                Save::Rsk => {
                    ui::loadnsave::save_to_file(self, &path);
                }
                Save::Csv => {
                    ui::loadnsave::save_1d_as_csv(&self.database,&self.err,self.len_h,self.len_v,&path).unwrap();
                }
            }
            
            
            Notification::new()
                .summary("File Saved")
                .body(format!("File saved to {}", path).as_str())
                .show().unwrap();
        }


        // Load dialog
        egui::Window::new("Load Spreadsheet")
        .open(&mut self.load_dialog)
        .order(egui::Order::Foreground)
        .fixed_size(egui::vec2(800.0, 500.0))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_sized([400.0,30.0],egui::TextEdit::singleline(&mut self.load_path).hint_text("Enter file path").font(FontId::proportional(20.0)));
                // ui.text_edit_singleline(&mut self.save_path);
                if ui.add_sized([90.0,30.0],Button::new(RichText::new("Browse").font(FontId::proportional(20.0)))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Rust Spreadsheet",&["rsk"]).pick_file() {
                        self.load_path = path.display().to_string();
                    }
                };
                
            });
            ui.add_space(10.0);


            ui.horizontal(|ui|{
                ui.label("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t");

                if ui.add_sized([100.0,30.0], Button::new(RichText::new("Load").font(FontId::proportional(20.0)))).clicked() {
                    self.load_todo = true;
                }

            });

        });

        if self.load_todo{
            self.load_dialog = false;
            self.load_todo = false;
            let path = self.load_path.clone();
            *self = ui::loadnsave::read_from_file(self.load_path.as_str());
            
            Notification::new()
                .summary("File Loaded")
                .body(format!("File Loaded from {}", path).as_str())
                .show().unwrap();
        }


        //  Plot dialog
        egui::Window::new("Plot Data")
        .open(&mut self.plot_dialog)
        .order(egui::Order::Foreground)
        .fixed_size(egui::vec2(800.0, 500.0))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
            ui.label(RichText::new("X-Axis:\t").font(FontId::proportional(20.0)));
            ui.add_sized([450.0, 30.0], egui::TextEdit::singleline(&mut self.plot_x_axis).hint_text("Enter column for X-axis").font(FontId::proportional(20.0)));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
            ui.label(RichText::new("Y-Axis:\t").font(FontId::proportional(20.0)));
            ui.add_sized([450.0, 30.0], egui::TextEdit::singleline(&mut self.plot_y_axis).hint_text("Enter column for Y-axis").font(FontId::proportional(20.0)));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
            ui.label(RichText::new("Rows: \t").font(FontId::proportional(20.0)));
            ui.add_sized([450.0, 30.0], egui::TextEdit::singleline(&mut self.plot_rows).hint_text("Enter row range (e.g., 1-10)").font(FontId::proportional(20.0)));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
            ui.label(RichText::new("Plot Type:\t\t").font(FontId::proportional(20.0)));
            if ui.add(egui::RadioButton::new(self.plot_type == Plot::Line, RichText::new("Line\t\t\t\t").font(FontId::proportional(20.0)))).clicked() {
                self.plot_type = Plot::Line;
            }
            if ui.add(egui::RadioButton::new(self.plot_type == Plot::Scatter, RichText::new("Scatter").font(FontId::proportional(20.0)))).clicked() {
                self.plot_type = Plot::Scatter;
            }
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Save Path:\t").font(FontId::proportional(20.0)));
                ui.add_sized([300.0, 30.0], egui::TextEdit::singleline(&mut self.plot_save).hint_text("Enter save path").font(FontId::proportional(20.0)));
                if ui.add_sized([90.0, 30.0], Button::new(RichText::new("Browse").font(FontId::proportional(20.0)))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("PNG Image", &["png"]).save_file() {
                        self.plot_save = path.display().to_string();
                    }
                };
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
            ui.label("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t");

            if ui.add_sized([100.0, 30.0], Button::new(RichText::new("Plot").font(FontId::proportional(20.0)))).clicked(){
                let mut data: Vec<(f64,f64)> = vec![];
                let rows: Vec<&str> = self.plot_rows.split(':').collect();
                if rows.len() == 2 {
                    if let (Ok(start), Ok(end)) = (rows[0].trim().parse::<i32>(), rows[1].trim().parse::<i32>()) {
                        if start <= end {
                            for i in start..=end {
                            data.push((self.database[crate::cell_to_ind(format!("{}{}",self.plot_x_axis,i).as_str(), self.len_h) as usize] as f64,
                            self.database[crate::cell_to_ind(format!("{}{}",self.plot_y_axis,i).as_str(), self.len_h) as usize] as f64
                         ));
                            }
                        }
                    } }
                
                if self.plot_type == Plot::Scatter {
                    utils::ui::plot::scatter_plot(&data,self.plot_save.as_str()).unwrap();
                }else{
                    utils::ui::plot::line_plot(&data,self.plot_save.as_str()).unwrap();
                }

                #[cfg(target_os = "windows")]
                {
                    // Windows: Use "start" to open the image
                    let _ = std::process::Command::new("cmd")
                        .args(["/C", "start", &self.plot_save])
                        .spawn()
                        .expect("Failed to open image").wait();
                }
                #[cfg(target_os = "linux")]
                {
                    // Linux: Use "xdg-open" to open the image
                    std::process::Command::new("xdg-open")
                        .arg(&self.plot_save)
                        .spawn()
                        .expect("Failed to open image");
                }
                
            self.plot_todo = true;


            };
            });
        });

        if self.plot_todo{
            self.plot_dialog = false;
            self.plot_todo = false;
        }


        // PDF dialog
        egui::Window::new("Save as PDF")
        .open(&mut self.pdf_dialog)
        .order(egui::Order::Foreground)
        .fixed_size(egui::vec2(800.0, 500.0))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_sized([400.0,30.0],egui::TextEdit::singleline(&mut self.pdf_path).hint_text("Enter PDF path").font(FontId::proportional(20.0)));
                // ui.text_edit_singleline(&mut self.save_path);
                if ui.add_sized([90.0,30.0],Button::new(RichText::new("Browse").font(FontId::proportional(20.0)))).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("PDF Document",&["pdf"]).save_file() {
                        self.pdf_path = path.display().to_string();
                    }
                };
                
            });
            ui.add_space(10.0);


            ui.horizontal(|ui|{
                ui.label("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t");

                if ui.add_sized([100.0,30.0], Button::new(RichText::new("Save").font(FontId::proportional(20.0)))).clicked() {
                    self.pdf_todo = true;
                }

            });

        });

        if self.pdf_todo{
            self.pdf_dialog = false;
            self.pdf_todo = false;
            utils::ui::loadnsave::save_1d_as_pdf(&self.database,&self.err,self.len_h,self.len_v,&self.pdf_path).unwrap();
            Notification::new()
                .summary("PDF Saved")
                .body(format!("PDF saved to {}", self.pdf_path).as_str())
                .show().unwrap();
        }


        // Describe dialog
        egui::Window::new("Describe Data")
        .open(&mut self.describe_dialog)
        .order(egui::Order::Foreground)
        .fixed_size(egui::vec2(400.0, 500.0))
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
            ui.label(RichText::new("Range:").font(FontId::proportional(20.0)));
            ui.add_sized([250.0, 30.0], egui::TextEdit::singleline(&mut self.describe_range).hint_text("Enter range (e.g., A1:B10)").font(FontId::proportional(20.0)));
            });

            ui.add_space(10.0);
            // let mut ans = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
            ui.horizontal(|ui| {
            ui.label("\t\t\t\t\t\t\t\t\t");
            
            if ui.add_sized([100.0, 30.0], Button::new(RichText::new("Describe").font(FontId::proportional(20.0)))).clicked() {
                let range = self.describe_range.clone();
                let mut start = 0;
                let mut end = 0;
                if range.contains(':'){
                    let parts: Vec<&str> = range.split(':').collect();
                    start = crate::cell_to_ind(parts[0],self.len_h);
                    end = crate::cell_to_ind(parts[1],self.len_h);
                }
                let n_cols= self.len_h;
                let mut y1=start/n_cols; let mut y2=end/n_cols;
                let mut x1 = start%(n_cols); if x1==0{x1=n_cols;}
                let mut x2 = end%(n_cols); if x2==0{x2=n_cols;}
                if x1!=n_cols{y1+=1;}
                if x2!=n_cols{y2+=1;}
                let mut data = Vec::new();
                for i in x1..x2+1{
                    for j in y1..y2+1{
                        data.push(self.database[(i + (j-1)*n_cols) as usize]);
                    }
                }
                self.describe_data = utils::ui::stats::calculate_stats(&data);
            }
            });
            ui.add_space(10.0);

            let labels = [
                "Count:",
                "Mean:",
                "Std Dev:",
                "Min:",
                "25%:",
                "50%:",
                "75%:",
                "Max:",
                // (count, mean, std, min, p25, p50, p75, max)
            ];

            for (i,item) in labels.iter().enumerate() {
            egui::Grid::new(format!("describe_grid_{}", i))
                .num_columns(2)
                .show(ui, |ui| {
                    egui::Frame::new()
                        
                            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                            .show(ui, |ui| {
                                ui.add_sized([100.0, 35.0], 

                                    egui::Label::new(RichText::new(item.to_string()).font(FontId::proportional(20.0)))
                                );
                                
                            });
                            egui::Frame::new()
                        
                            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                            .show(ui, |ui| {
                                ui.add_sized([200.0, 35.0], 

                                    egui::Label::new(RichText::new(format!("{}", self.describe_data[i])).font(FontId::proportional(20.0)))
                                );
                                
                            });
                    ui.end_row();
                });
            ui.add_space(10.0);
            }
        });
        
        // About dialog
        egui::Window::new("About Rust Spreadsheet")
            .open(&mut self.about_dialog)
            .order(egui::Order::Foreground)
            .fixed_size(egui::vec2(600.0, 400.0))
            .collapsible(false)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("Rust Spreadsheet Project").font(FontId::proportional(30.0)).color(Color32::from_rgb(0, 120, 215)));
                ui.add_space(10.0);
                ui.label(RichText::new("Version: 1.0.0").font(FontId::proportional(20.0)));
                ui.add_space(10.0);
                ui.label(RichText::new("Developed by:").font(FontId::proportional(20.0)));
                ui.label(RichText::new("Rajat Soni: 2023CS10229").font(FontId::proportional(18.0)));
                ui.label(RichText::new("Krish Bhiamni: 2023CS10712").font(FontId::proportional(18.0)));
                ui.label(RichText::new("Priyanshu Gaurav").font(FontId::proportional(18.0)));
                ui.label(RichText::new("IIT Delhi, Semester 4, COP290").font(FontId::proportional(18.0)));
                ui.add_space(10.0);
                ui.label(RichText::new("Description:").font(FontId::proportional(20.0)));
                ui.label(RichText::new("This project is a spreadsheet application built purely using rust and the eframe/egui library. It supports various features such as saving, loading, plotting, and statistical analysis.").font(FontId::proportional(18.0)));
                ui.add_space(10.0);
                // ui.label(RichText::new("Contact:").font(FontId::proportional(20.0)));
                // ui.label(RichText::new("Email: rustspreadsheet@iitd.ac.in").font(FontId::proportional(18.0)));
                
            });
        

        egui::CentralPanel::default().show(ctx, |ui| {
            
            // if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)){
            //     self.top_v-=1;
            // }

            // if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)){
            //     self.top_h-=1;
            // }

            // if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)){
            //     self.top_h+=1;
            // }

            // if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)){
            //     self.top_v+=1;
            // }

            ui.add_space(10.0);
            // Header
            ui.horizontal(|ui| {
                // ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/copy.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 })));
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/info.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked(){
                    self.about_dialog = true;
                };
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/describe.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked(){
                    self.describe_dialog = true;
                };
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/plot.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked(){
                    self.plot_dialog = true;
                };
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/pdf.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked(){
                    self.pdf_dialog = true;
                };
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/folder.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked(){
                    self.load_dialog = true;
                };
                if ui.add_sized([120.0,100.0],egui::Button::image(egui::Image::new(egui::include_image!("assets/save.png")).fit_to_exact_size(egui::Vec2 { x: 100.0, y: 80.0 }))).clicked() {
                    self.save_dialog = true;
                };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let current_date = chrono::Local::now().format("%A, %B %d, %Y").to_string();
                    let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
                    ui.add_sized([310.0, 80.0], egui::Label::new(RichText::new(format!("Rust Spreadsheet Project\n\nDate: {}\nTime: {}",current_date,current_time)).font(FontId::proportional(20.0))));
            });});
            ui.horizontal(|ui| {
                // ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("Copy").font(FontId::proportional(15.0))));
                ui.add_sized([120.0,4.0],egui::Label::new(RichText::new("About").font(FontId::proportional(15.0))));
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
                    if self.selected_cell.is_some() {
                        self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.selected_cell.unwrap()%self.len_h), self.selected_cell.unwrap()/self.len_h + 1);
                    }else{
                        
                        self.cell_ref.0 = String::new();
                        
                    }
                    
                    self.cell_ref.1 = true;
                }

                if cell.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let temp = format!("scroll_to {}", self.cell_ref.0);
                    let out = utils::input::input(&temp, self.len_h, self.len_v);
                    let status = out[4].clone();
                    if status == "ok" && out[1] == "SRL" {   
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
                    self.cell_ref.1 = false;
                };
            }else{

                
                
                if self.selected_cell.is_some() {
                    self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.selected_cell.unwrap()%self.len_h), self.selected_cell.unwrap()/self.len_h + 1);
                }else if self.hovered_cell.is_some(){
                    self.cell_ref.0 = format!("{}{}",utils::display::get_label(self.hovered_cell.unwrap()%self.len_h), self.hovered_cell.unwrap()/self.len_h + 1);
                }else{
                self.cell_ref.0 = String::new();
                }
                
                
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                    .show(ui, |ui| {
                        let cell = ui.add_sized([210.0,30.0], egui::Label::new(RichText::new(self.cell_ref.0.to_string()).font(FontId::proportional(20.0))));
                        
                        
                        if cell.clicked() {
                                            self.cell_ref.1 = true;
                                        }


                    });

                
            }

            
            


            egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
            .show(ui, |ui| {
                ui.add_sized([950.0,30.0], egui::Label::new(RichText::new(self.temp_txt.0.to_string()).font(FontId::proportional(20.0))));
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

                                    egui::Label::new(RichText::new(String::new()).font(FontId::proportional(20.0)))
                                );
                                
                            });
                    for col in 0..10 {
                            
                        egui::Frame::new()
                        
                            .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                            .show(ui, |ui| {
                                ui.add_sized([100.0, 35.0], 

                                    egui::Label::new(RichText::new(utils::display::get_label(col+self.top_h).to_string()).font(FontId::proportional(20.0)))
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
                                    if self.selected_cell.is_none() || (self.selected_cell.unwrap() != ind) {
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
                                        self.temp_txt.0 = self.formula[ind as usize].to_string();
                                        
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
                                        if status == "ok" && out[1] != "SRL" {   
                                            let suc = crate::cell_update(&out, &mut self.database, &mut self.sensi, &mut self.opers, self.len_h, &mut self.indegree, &mut self.err);
                                            if suc==0{
                                                // Write code for error here
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
