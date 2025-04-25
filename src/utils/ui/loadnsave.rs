//! File handling utilities for the spreadsheet application.
//!
//! This module provides functions to save and load spreadsheet data in different formats:
//! - Native format (.rsk) for preserving all spreadsheet state using JSON serialization
//! - CSV export for compatibility with other spreadsheet applications
//! - PDF export for creating printable documents from spreadsheet data
//!
//! The module handles serialization and deserialization of the spreadsheet state and
//! creation of formatted output files.

use crate::utils::ui;
use csv::Writer;
use genpdf::{Document, Element, elements};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Saves spreadsheet data to a file in the native format (.rsk).
///
/// This function serializes the entire spreadsheet state to JSON and writes it to the specified path.
/// The native format preserves all application state including formulas, cell relationships,
/// and UI settings.
///
/// # Arguments
/// * `data` - Mutable reference to the spreadsheet to be saved
/// * `path` - Path where the file will be saved
pub fn save_to_file(data: &mut ui::gui::Spreadsheet, path: &str) {
    let json_data = serde_json::to_string(data).expect("Failed to serialize data");

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    println!("Data saved successfully to {}", path);
}

/// Reads spreadsheet data from a file in the native format (.rsk).
///
/// This function reads a JSON file and deserializes it into a Spreadsheet struct,
/// restoring the complete application state.
///
/// # Arguments
/// * `path` - Path to the file to be read
///
/// # Returns
/// A new Spreadsheet instance with the loaded data
pub fn read_from_file(path: &str) -> ui::gui::Spreadsheet {
    let file_content = std::fs::read_to_string(path).expect("Failed to read file");
    let spreadsheet: ui::gui::Spreadsheet =
        serde_json::from_str(&file_content).expect("Failed to deserialize data");

    println!("Data loaded successfully from {}", path);
    spreadsheet
}

/// Exports spreadsheet data to a CSV file.
///
/// This function creates a CSV file containing the visible values from the spreadsheet.
/// Cells with errors are marked with "ERR".
///
/// # Arguments
/// * `data` - Slice containing cell values
/// * `err` - Slice indicating which cells have errors
/// * `len_h` - Number of columns in the spreadsheet
/// * `len_v` - Number of rows in the spreadsheet
/// * `filename` - Path where the CSV file will be saved
///
/// # Returns
/// `Ok(())` if the operation was successful, or an error otherwise
pub fn save_1d_as_csv(
    data: &[i32],
    err: &[bool],
    len_h: i32,
    len_v: i32,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;

    for j in 1..=len_v {
        let mut ans = vec![String::new(); len_h as usize];
        for i in 1..=len_h {
            let index: usize = ((j - 1) * len_h + i) as usize;
            if err[index] {
                ans[(i - 1) as usize] = "ERR".to_string();
            } else {
                ans[(i - 1) as usize] = data[index].to_string();
            }
        }
        wtr.write_record(ans)?;
    }

    wtr.flush()?;
    Ok(())
}

/// Exports spreadsheet data to a PDF file.
///
/// This function creates a formatted PDF document representing the spreadsheet content.
/// The PDF includes proper pagination for large spreadsheets, with each page showing up to
/// 10x10 cells. Cells with errors are marked with "ERR".
///
/// # Arguments
/// * `data` - Slice containing cell values
/// * `err` - Slice indicating which cells have errors
/// * `len_h` - Number of columns in the spreadsheet
/// * `len_v` - Number of rows in the spreadsheet
/// * `filename` - Path where the PDF file will be saved
///
/// # Returns
/// `Ok(())` if the operation was successful, or an error otherwise
pub fn save_1d_as_pdf(
    data: &[i32],
    err: &[bool],
    len_h: i32,
    len_v: i32,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // Load font
    // println!("{:?}", std::fs::canonicalize("./src/utils/ui/assets/ARIAL.ttf"));
    let font = genpdf::fonts::from_files("./src/utils/ui/assets", "ARIAL", None)?;

    let mut doc = Document::new(font);
    doc.set_title("1D Grid Export");
    doc.set_paper_size(genpdf::Size::new(841.89, 595.28));
    doc.set_line_spacing(2.0);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(genpdf::Margins::trbl(50.0, 20.0, 20.0, 20.0));

    let mut style = genpdf::style::Style::new();
    style.set_font_size(45);

    doc.set_page_decorator(decorator);
    // Set up table layout

    let mut pages = 1;
    let hz = (len_h as f64 / 10.0).ceil() as i32;
    let vz = (len_v as f64 / 10.0).ceil() as i32;
    let total_pages = hz * vz;
    for top_h in 0..hz {
        for top_v in 0..vz {
            let mut table = elements::TableLayout::new(vec![1; 10_usize]);
            table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
            for j in 1..=10 {
                let mut row = table.row();
                // let mut row = Vec::with_capacity(len_h as usize);
                for i in 1..=10 {
                    let index = if top_h * 10 + i > len_h || top_v * 10 + j > len_v {
                        0
                    } else {
                        ((top_v * 10 + j - 1) * len_h + i + top_h * 10) as usize
                    };
                    let cell = if err[index] {
                        "ERR".to_string()
                    } else {
                        data[index].to_string()
                    };
                    row.push_element(
                        elements::Paragraph::new("")
                            .styled_string(cell, style)
                            .padded(15.0),
                    );
                }
                row.push()?;
            }
            doc.push(table);
            doc.push(
                elements::Paragraph::new(format!(
                    "Page {} of {}, Displaying - {}{} to {}{}",
                    pages,
                    total_pages,
                    crate::utils::display::get_label(top_h * 10 + 1),
                    top_v * 10 + 1,
                    crate::utils::display::get_label(top_h * 10 + 10),
                    top_v * 10 + 10
                ))
                .styled(style),
            );
            pages += 1;
            if pages <= total_pages {
                doc.push(elements::PageBreak::new());
            }
        }
    }

    // Fill table rows

    // Add to document and render

    doc.render_to_file(filename)?;

    println!("PDF saved to {}", filename);
    Ok(())
}
