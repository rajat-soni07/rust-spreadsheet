use std::fs::File;
use std::io::Write;
use crate::utils::ui;
use csv::Writer;
use std::error::Error;



pub fn save_to_file(data: &mut ui::ui::Spreadsheet,path: &str){

    let json_data = serde_json::to_string(data).expect("Failed to serialize data");

    
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");

    println!("Data saved successfully to {}", path);
}

pub fn read_from_file(path: &str) -> ui::ui::Spreadsheet {
    let file_content = std::fs::read_to_string(path).expect("Failed to read file");
    let spreadsheet: ui::ui::Spreadsheet = serde_json::from_str(&file_content).expect("Failed to deserialize data");

    println!("Data loaded successfully from {}", path);
    spreadsheet
}

pub fn save_1d_as_csv(data: &Vec<i32>,err: &Vec<bool>, len_h: i32, len_v:i32, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;

    for j in 1..=len_v {
        let mut ans = vec![String::new();len_h as usize];
        for i in 1..=len_h {
            let index:usize = ((j - 1) * len_h + i) as usize;
            if err[index]{
                ans[(i-1) as usize ] = "ERR".to_string();
            }else{
                ans[(i-1) as usize ] = data[index].to_string();
            }
        }
        wtr.write_record(ans)?;
    }

    wtr.flush()?;
    Ok(())
}