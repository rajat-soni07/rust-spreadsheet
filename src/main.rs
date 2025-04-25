//! # Rust Spreadsheet Application
//!
//! This is the main entry point for the Rust Spreadsheet application, which provides
//! both a terminal-based and graphical user interface for a spreadsheet-like grid system.
//!
//! The application supports:
//! - Formula-based cell calculations with dependency tracking
//! - Cycle detection in cell references
//! - Various operations including arithmetic, statistical functions, and time delays
//! - Both terminal and graphical user interfaces

use std::io;
use std::io::Write;

mod utils;

/// Represents an operation to be performed on a cell.
///
/// # Fields
///
/// * `opcpde` - Operation code specifying what calculation to perform
/// * `cell1` - First operand (either a cell reference or direct value)
/// * `cell2` - Second operand (either a cell reference or direct value)
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Ops {
    opcpde: String,
    cell1: i32,
    cell2: i32,
}
impl Clone for Ops {
    fn clone(&self) -> Self {
        Ops {
            opcpde: self.opcpde.clone(),
            cell1: self.cell1,
            cell2: self.cell2,
        }
    }
}

/// Returns the maximum of two integers.
///
/// # Arguments
///
/// * `a` - First integer
/// * `b` - Second integer
///
/// # Returns
///
/// The larger of the two input values
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Converts a cell reference string (like "A1") to an integer representation.
///
/// # Arguments
///
/// * `a` - Cell reference string (e.g., "A1", "B2", etc.)
///
/// # Returns
///
/// An integer representation where column is multiplied by 1000 and added to row
fn cell_to_int(a: &str) -> i32 {
    let mut col = 0;
    let b = a.chars();
    let mut part = 0;
    for c in b.clone() {
        if c.is_alphabetic() {
            part += 1;
        } else {
            break;
        }
    }

    for i in a[..part].chars() {
        let diff = i as i32 - 'A' as i32 + 1;

        if (1..=26).contains(&diff) {
            col *= 26;
            col += diff;
        } else {
            break;
        }
    }

    let row: i32 = a[part..].parse().unwrap_or(0);

    col * 1000 + row
}

/// Converts an integer cell representation to a linear index in the spreadsheet array.
///
/// # Arguments
///
/// * `a` - Integer representation of a cell
/// * `len_h` - Width of the spreadsheet (number of columns)
///
/// # Returns
///
/// Linear index in the spreadsheet array
fn int_to_ind(a: i32, len_h: i32) -> i32 {
    (a / 1000) + (a % 1000 - 1) * len_h
}

/// Converts a cell reference string directly to a linear index in the spreadsheet array.
///
/// # Arguments
///
/// * `a` - Cell reference string (e.g., "A1", "B2", etc.)
/// * `len_h` - Width of the spreadsheet (number of columns)
///
/// # Returns
///
/// Linear index in the spreadsheet array
fn cell_to_ind(a: &str, len_h: i32) -> i32 {
    int_to_ind(cell_to_int(a), len_h)
}

/// Calculates the value of a cell based on its operation and dependencies.
///
/// # Arguments
///
/// * `cell` - Index of the cell to calculate
/// * `database` - Mutable reference to the array of cell values
/// * `opers` - Slice of operations for each cell
/// * `len_h` - Width of the spreadsheet (number of columns)
/// * `err` - Mutable reference to the array tracking cell errors
fn calc(cell: i32, database: &mut [i32], opers: &[Ops], len_h: i32, err: &mut [bool]) {
    match opers[cell as usize].opcpde.as_str() {
        "CCA" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] + database[cell2];
        }
        "CVA" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] + opers[cell as usize].cell2;
        }
        "VCA" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = database[cell2] + opers[cell as usize].cell1;
        }
        "VVA" => {
            database[cell as usize] = opers[cell as usize].cell1 + opers[cell as usize].cell2;
        }
        "CCS" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] - database[cell2];
        }
        "CVS" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] - opers[cell as usize].cell2;
        }
        "VCS" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = opers[cell as usize].cell1 - database[cell2];
        }
        "VVS" => {
            database[cell as usize] = opers[cell as usize].cell1 - opers[cell as usize].cell2;
        }
        "CCM" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] * database[cell2];
        }
        "CVM" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] * opers[cell as usize].cell2;
        }
        "VCM" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = opers[cell as usize].cell1 * database[cell2];
        }
        "VVM" => {
            database[cell as usize] = opers[cell as usize].cell1 * opers[cell as usize].cell2;
        }
        "CCD" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2] || database[cell2] == 0;
            if database[cell2] != 0 {
                database[cell as usize] = database[cell1] / database[cell2];
            }
        }
        "CVD" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1] || opers[cell as usize].cell2 == 0;
            if opers[cell as usize].cell2 != 0 {
                database[cell as usize] = database[cell1] / opers[cell as usize].cell2;
            }
        }
        "VCD" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2] || database[cell2] == 0;
            if database[cell2] != 0 {
                database[cell as usize] = opers[cell as usize].cell1 / database[cell2];
            }
        }
        "VVD" => {
            err[cell as usize] = opers[cell as usize].cell2 == 0;
            if opers[cell as usize].cell2 != 0 {
                database[cell as usize] = opers[cell as usize].cell1 / opers[cell as usize].cell2;
            }
        }
        "EQC" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1];
        }
        "EQV" => {
            err[cell as usize] = false;
            database[cell as usize] = opers[cell as usize].cell1;
        }
        "MIN" => {
            database[cell as usize] = utils::operations::min(
                opers[cell as usize].cell1,
                opers[cell as usize].cell2,
                database,
                len_h,
                err,
                cell,
            );
        }
        "MAX" => {
            database[cell as usize] = utils::operations::max(
                opers[cell as usize].cell1,
                opers[cell as usize].cell2,
                database,
                len_h,
                err,
                cell,
            );
        }
        "MEA" => {
            database[cell as usize] = utils::operations::avg(
                opers[cell as usize].cell1,
                opers[cell as usize].cell2,
                database,
                len_h,
                err,
                cell,
            );
        }
        "SUM" => {
            database[cell as usize] = utils::operations::sum(
                opers[cell as usize].cell1,
                opers[cell as usize].cell2,
                database,
                len_h,
                err,
                cell,
            );
        }
        "STD" => {
            database[cell as usize] = utils::operations::stdev(
                opers[cell as usize].cell1,
                opers[cell as usize].cell2,
                database,
                len_h,
                err,
                cell,
            );
        }
        "SLV" => {
            std::thread::sleep(std::time::Duration::from_secs(
                max(0, opers[cell as usize].cell1) as u64,
            ));
            database[cell as usize] = opers[cell as usize].cell1;
            err[cell as usize] = false;
        }
        "SLC" => {
            if err[opers[cell as usize].cell1 as usize] {
                err[cell as usize] = true;
            } else {
                std::thread::sleep(std::time::Duration::from_secs(max(
                    0,
                    database[opers[cell as usize].cell1 as usize],
                ) as u64));
                database[cell as usize] = database[opers[cell as usize].cell1 as usize];
                err[cell as usize] = false;
            }
        }
        _ => {}
    }
}

/// Updates cell values according to a topological ordering of dependencies.
///
/// # Arguments
///
/// * `topo_arr` - Topologically sorted array of cell indices
/// * `database` - Mutable reference to the array of cell values
/// * `opers` - Slice of operations for each cell
/// * `len_h` - Width of the spreadsheet (number of columns)
/// * `err` - Mutable reference to the array tracking cell errors
fn val_update(topo_arr: &[i32], database: &mut [i32], opers: &[Ops], len_h: i32, err: &mut [bool]) {
    for i in 1..=topo_arr[0] {
        calc(topo_arr[i as usize], database, opers, len_h, err)
    }
}

/// Updates a cell with a new operation and recalculates dependent cells.
///
/// This function handles the dependency tracking, cycle detection, and propagation
/// of changes through the spreadsheet.
///
/// # Arguments
///
/// * `inp_arr` - Input array containing cell reference and operation details
/// * `database` - Mutable reference to the array of cell values
/// * `sensi` - Mutable reference to the sensitivity list for dependency tracking
/// * `opers` - Mutable reference to the array of cell operations
/// * `len_h` - Width of the spreadsheet (number of columns)
/// * `indegree` - Mutable reference to the array tracking in-degrees for cycle detection (used in toposort)
/// * `err` - Mutable reference to the array tracking cell errors
///
/// # Returns
///
/// 1 if update was successful, 0 if a cycle was detected
fn cell_update(
    inp_arr: &[String],
    database: &mut [i32],
    sensi: &mut [Vec<i32>],
    opers: &mut [Ops],
    len_h: i32,
    indegree: &mut [i32],
    err: &mut [bool],
) -> i32 {
    let target = cell_to_ind(&inp_arr[0], len_h);
    let target = target as usize;
    // Storing temporary value of opers in case a cycle is present
    let rev = Ops {
        opcpde: opers[target].opcpde.clone(),
        ..opers[target]
    };

    // Copying data to opers
    opers[target].opcpde = inp_arr[1].clone();
    if let Ok(value) = inp_arr[2].parse::<i32>() {
        opers[target].cell1 = value;
    } else {
        opers[target].cell1 = cell_to_ind(&inp_arr[2], len_h);
    }

    if let Ok(value) = inp_arr[3].parse::<i32>() {
        opers[target].cell2 = value;
    } else {
        opers[target].cell2 = cell_to_ind(&inp_arr[3], len_h);
    }

    //Removing older values from sensitivity list

    // Handling arithmetic
    if rev.opcpde.starts_with('C') {
        sensi[rev.cell1 as usize].retain(|&x| x != target as i32);
    }

    if rev.opcpde.chars().nth(1) == Some('C') {
        sensi[rev.cell2 as usize].retain(|&x| x != target as i32);
    }

    // Handling eq
    if rev.opcpde == "EQC" {
        sensi[rev.cell1 as usize].retain(|&x| x != target as i32);
    }

    // Handling sleep
    if rev.opcpde == "SLC" {
        sensi[rev.cell1 as usize].retain(|&x| x != target as i32);
    }

    // Handling ranges
    if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
        let mut x1 = (rev.cell1 % len_h) as usize;
        let mut x2 = (rev.cell2 % len_h) as usize;
        if x1 == 0 {
            x1 = len_h as usize;
        }
        if x2 == 0 {
            x2 = len_h as usize;
        }

        let y1 = (rev.cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
        let y2 = (rev.cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
            let mut xx1 = (opers[target].cell1 % len_h) as usize;
            let mut xx2 = (opers[target].cell2 % len_h) as usize;
            if xx1 == 0 {
                xx1 = len_h as usize;
            }
            if xx2 == 0 {
                xx2 = len_h as usize;
            }

            let xy1 = (opers[target].cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
            let xy2 = (opers[target].cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);

            for i in x1..=x2 {
                for j in y1..=y2 {
                    if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                        sensi[i + (j - 1) * len_h as usize].retain(|&x| x != target as i32);
                    }
                }
            }
        } else {
            for i in x1..=x2 {
                for j in y1..=y2 {
                    sensi[i + (j - 1) * len_h as usize].retain(|&x| x != target as i32);
                }
            }
        }
    }

    // Adding items to sensitivity list

    // Handling arithmetic
    if inp_arr[1].starts_with('C')
        && (sensi[opers[target].cell1 as usize].is_empty()
            || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32)
    {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    if inp_arr[1].chars().nth(1) == Some('C')
        && (sensi[opers[target].cell2 as usize].is_empty()
            || *sensi[opers[target].cell2 as usize].last().unwrap() != target as i32)
    {
        sensi[opers[target].cell2 as usize].push(target as i32);
    }

    // Handling eq
    if inp_arr[1] == "EQC"
        && (sensi[opers[target].cell1 as usize].is_empty()
            || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32)
    {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    if inp_arr[1] == "SLC"
        && (sensi[opers[target].cell1 as usize].is_empty()
            || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32)
    {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    // Handling ranges
    if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
        let mut x1 = (opers[target].cell1 % len_h) as usize;
        let mut x2 = (opers[target].cell2 % len_h) as usize;
        if x1 == 0 {
            x1 = len_h as usize;
        }
        if x2 == 0 {
            x2 = len_h as usize;
        }

        let y1 = (opers[target].cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
        let y2 = (opers[target].cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
            let mut xx1 = (rev.cell1 % len_h) as usize;
            let mut xx2 = (rev.cell2 % len_h) as usize;
            if xx1 == 0 {
                xx1 = len_h as usize;
            }
            if xx2 == 0 {
                xx2 = len_h as usize;
            }

            let xy1 = (rev.cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
            let xy2 = (rev.cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);

            for i in x1..=x2 {
                for j in y1..=y2 {
                    if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                        sensi[i + (j - 1) * len_h as usize].push(target as i32);
                    }
                }
            }
        } else {
            for i in x1..=x2 {
                for j in y1..=y2 {
                    sensi[i + (j - 1) * len_h as usize].push(target as i32);
                }
            }
        }
    }

    let topo = utils::toposort::topo_sort(sensi, target as i32, indegree);

    if topo[0] == -1 {
        // Removing items from sensitivity list

        // Handling arithmetic
        if inp_arr[1].starts_with('C') {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].pop();
                }
            }
        }

        if inp_arr[1].chars().nth(1) == Some('C') {
            if let Some(first) = sensi[opers[target].cell2 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell2 as usize].pop();
                }
            }
        }

        // Handling eq
        if inp_arr[1] == "EQC" {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].pop();
                }
            }
        }

        // Handling sleep
        if inp_arr[1] == "SLC" {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].pop();
                }
            }
        }

        // Handling ranges
        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
            let mut x1 = (opers[target].cell1 % len_h) as usize;
            let mut x2 = (opers[target].cell2 % len_h) as usize;
            if x1 == 0 {
                x1 = len_h as usize;
            }
            if x2 == 0 {
                x2 = len_h as usize;
            }

            let y1 = (opers[target].cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
            let y2 = (opers[target].cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
                let mut xx1 = (rev.cell1 % len_h) as usize;
                let mut xx2 = (rev.cell2 % len_h) as usize;
                if xx1 == 0 {
                    xx1 = len_h as usize;
                }
                if xx2 == 0 {
                    xx2 = len_h as usize;
                }

                let xy1 = (rev.cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
                let xy2 = (rev.cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);

                for i in x1..=x2 {
                    for j in y1..=y2 {
                        if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                            sensi[i + (j - 1) * len_h as usize].pop();
                        }
                    }
                }
            } else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        sensi[i + (j - 1) * len_h as usize].pop();
                    }
                }
            }
        }

        // Adding back older values

        if rev.opcpde.starts_with('C')
            && (sensi[rev.cell1 as usize].is_empty()
                || *sensi[rev.cell1 as usize].last().unwrap() != target as i32)
        {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        if rev.opcpde.chars().nth(1) == Some('C')
            && (sensi[rev.cell2 as usize].is_empty()
                || *sensi[rev.cell2 as usize].last().unwrap() != target as i32)
        {
            sensi[rev.cell2 as usize].push(target as i32);
        }

        // Handling eq
        if rev.opcpde == "EQC"
            && (sensi[rev.cell1 as usize].is_empty()
                || *sensi[rev.cell1 as usize].last().unwrap() != target as i32)
        {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        // Handling sleep
        if rev.opcpde == "SLC"
            && (sensi[rev.cell1 as usize].is_empty()
                || *sensi[rev.cell1 as usize].last().unwrap() != target as i32)
        {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        // Handling ranges
        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
            let mut x1 = (rev.cell1 % len_h) as usize;
            let mut x2 = (rev.cell2 % len_h) as usize;
            if x1 == 0 {
                x1 = len_h as usize;
            }
            if x2 == 0 {
                x2 = len_h as usize;
            }

            let y1 = (rev.cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
            let y2 = (rev.cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
                let mut xx1 = (opers[target].cell1 % len_h) as usize;
                let mut xx2 = (opers[target].cell2 % len_h) as usize;
                if xx1 == 0 {
                    xx1 = len_h as usize;
                }
                if xx2 == 0 {
                    xx2 = len_h as usize;
                }

                let xy1 =
                    (opers[target].cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
                let xy2 =
                    (opers[target].cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);

                for i in x1..=x2 {
                    for j in y1..=y2 {
                        if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                            sensi[i + (j - 1) * len_h as usize].push(target as i32);
                        }
                    }
                }
            } else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        sensi[i + (j - 1) * len_h as usize].push(target as i32);
                    }
                }
            }
        }

        // Restoring back previous ops in case of cycle
        opers[target] = Ops {
            opcpde: rev.opcpde.clone(),
            ..rev
        };

        0
    } else {
        val_update(&topo, database, opers, len_h, err);
        1
    }
}

/// Runs the terminal-based user interface for the spreadsheet.
///
/// # Arguments
///
/// * `len_h` - Width of the spreadsheet (number of columns)
/// * `len_v` - Height of the spreadsheet (number of rows)
fn non_ui(len_h: i32, len_v: i32) {
    let mut database = vec![0; (len_h * len_v + 1) as usize];
    let mut err = vec![false; (len_h * len_v + 1) as usize];
    let mut opers = vec![
        Ops {
            opcpde: String::new(),
            cell1: -1,
            cell2: -1
        };
        (len_h * len_v + 1) as usize
    ];
    let mut indegree = vec![0; (len_h * len_v + 1) as usize];
    let mut sensi = vec![Vec::<i32>::new(); (len_h * len_v + 1) as usize];

    let mut curr_h = 1;
    let mut curr_v = 1;
    let mut status = String::from("ok");
    let mut dis = false;

    utils::display::display_grid(curr_h, curr_v, len_h, len_v, &database, &err);

    let mut time = 0.0;
    loop {
        print!("[{:.1}] ({}) > ", time, status);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim_end().to_string();
        let start_time = std::time::Instant::now();
        match input.as_str() {
            "w" => {
                curr_v = max(1, curr_v - 10);
            }
            "a" => {
                curr_h = max(1, curr_h - 10);
            }
            "s" => {
                if curr_v + 10 >= len_v {
                    curr_v = len_v - 9
                } else {
                    curr_v += 10
                }
            }
            "d" => {
                if curr_h + 10 >= len_h {
                    curr_h = len_h - 9
                } else {
                    curr_h += 10
                }
            }
            "q" => {
                break;
            }
            "disable_output" => {
                dis = true;
                status = "ok".to_string();
            }
            "enable_output" => {
                status = "ok".to_string();
                dis = false;
            }
            _ => {
                let out = utils::input::input(&input, len_h, len_v);
                status = out[4].clone();
                if status == "ok" {
                    if out[1] == "SRL" {
                        let t = cell_to_ind(out[0].as_str(), len_h);
                        let mut x1 = t % len_h;
                        if x1 == 0 {
                            x1 = len_h;
                        }
                        let y1 = t / len_h + ((x1 != len_h) as i32);
                        curr_h = x1;
                        curr_v = y1;
                        // println!("Scrolling to cell {} at ({},{})", out[0], curr_h, curr_v);
                    } else {
                        let suc = cell_update(
                            &out,
                            &mut database,
                            &mut sensi,
                            &mut opers,
                            len_h,
                            &mut indegree,
                            &mut err,
                        );
                        if suc == 0 {
                            status = "cycle_detected".to_string();
                        }
                    }
                }
            }
        }
        let end_time = std::time::Instant::now();
        time = (end_time - start_time).as_secs_f64();

        if dis {
            continue;
        } else {
            utils::display::display_grid(curr_h, curr_v, len_h, len_v, &database, &err);
        }
    }
}

/// Main entry point for the application.
///
/// Parses command line arguments and launches either the terminal-based
/// or graphical user interface with the specified dimensions.
///
/// # Command Line Arguments
///
/// * First argument: Number of rows
/// * Second argument: Number of columns
/// * Third argument (optional): "--ui" to launch the graphical interface
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 {
        let len_h: i32 = args[2].parse().unwrap_or(10);
        let len_v: i32 = args[1].parse().unwrap_or(10);
        if args.len() == 4 {
            if args[3] == "--ui" {
                crate::utils::ui::gui::ui(len_h, len_v).unwrap();
            }
        } else {
            non_ui(len_h, len_v);
        }
    } else {
        println!("Usage: cargo run <len_h> <len_v> <flag>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max(5, 3), 5);
        assert_eq!(max(-5, -3), -3);
        assert_eq!(max(0, 0), 0);
    }

    #[test]
    fn test_cell_to_int() {
        assert_eq!(cell_to_int("A1"), 1001);
        assert_eq!(cell_to_int("B5"), 2005);
        assert_eq!(cell_to_int("Z10"), 26010);
        assert_eq!(cell_to_int("AA1"), 27001);
    }

    #[test]
    fn test_int_to_ind() {
        assert_eq!(int_to_ind(1001, 10), 1); // A1 in 10x10 grid
        assert_eq!(int_to_ind(2005, 10), 2 + (5 - 1) * 10); // B5 in 10x10 grid
        assert_eq!(int_to_ind(3003, 5), 3 + (3 - 1) * 5); // C3 in 5x5 grid
    }

    #[test]
    fn test_cell_to_ind() {
        assert_eq!(cell_to_ind("A1", 10), 1);
        assert_eq!(cell_to_ind("B5", 10), 2 + (5 - 1) * 10);
        assert_eq!(cell_to_ind("C3", 5), 3 + (3 - 1) * 5);
    }

    #[test]
    fn test_calc_basic_arithmetic() {
        let mut database = vec![0, 10, 5, 0]; // Index 0 unused, A1=10, B1=5, C1=0
        let mut err = vec![false, false, false, false];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            }, // B1 = 5
            Ops {
                opcpde: String::from("VVA"),
                cell1: 7,
                cell2: 3,
            }, // C1 = 7 + 3
        ];

        calc(3, &mut database, &opers, 3, &mut err);
        assert_eq!(database[3], 10); // 7 + 3 = 10
        assert!(!err[3]);
    }

    #[test]
    fn test_calc_all_arithmetics() {
        let mut database = vec![0, 10, 5, 0, 0, 0, 0, 0, 0]; // Index 0 unused, A1=10, B1=5, rest are results
        let mut err = vec![false; 9];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            }, // B1 = 5
            Ops {
                opcpde: String::from("CCA"),
                cell1: 1,
                cell2: 2,
            }, // C1 = A1 + B1 = 15
            Ops {
                opcpde: String::from("CCS"),
                cell1: 1,
                cell2: 2,
            }, // D1 = A1 - B1 = 5
            Ops {
                opcpde: String::from("CCM"),
                cell1: 1,
                cell2: 2,
            }, // E1 = A1 * B1 = 50
            Ops {
                opcpde: String::from("CCD"),
                cell1: 1,
                cell2: 2,
            }, // F1 = A1 / B1 = 2
            Ops {
                opcpde: String::from("VVM"),
                cell1: 3,
                cell2: 4,
            }, // G1 = 3 * 4 = 12
            Ops {
                opcpde: String::from("CVS"),
                cell1: 1,
                cell2: 2,
            }, // H1 = A1 - 2 = 8
        ];

        for i in 3..=8 {
            calc(i, &mut database, &opers, 3, &mut err);
        }

        assert_eq!(database[3], 15); // CCA: A1 + B1 = 10 + 5 = 15
        assert_eq!(database[4], 5); // CCS: A1 - B1 = 10 - 5 = 5
        assert_eq!(database[5], 50); // CCM: A1 * B1 = 10 * 5 = 50
        assert_eq!(database[6], 2); // CCD: A1 / B1 = 10 / 5 = 2
        assert_eq!(database[7], 12); // VVM: 3 * 4 = 12
        assert_eq!(database[8], 8); // CVS: A1 - 2 = 10 - 2 = 8
    }

    #[test]
    fn test_calc_specialized_operations() {
        let mut database = vec![0, 10, 20, 30, 40, 0, 0]; // Index 0 unused, A1=10, B1=20, C1=30, D1=40
        let mut err = vec![false; 7];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 20,
                cell2: -1,
            }, // B1 = 20
            Ops {
                opcpde: String::from("EQV"),
                cell1: 30,
                cell2: -1,
            }, // C1 = 30
            Ops {
                opcpde: String::from("EQV"),
                cell1: 40,
                cell2: -1,
            }, // D1 = 40
            Ops {
                opcpde: String::from("EQC"),
                cell1: 3,
                cell2: -1,
            }, // E1 = C1 = 30
            Ops {
                opcpde: String::from("SLC"),
                cell1: 1,
                cell2: -1,
            }, // F1 = sleep(A1) then A1 = 10
        ];

        calc(5, &mut database, &opers, 4, &mut err); // EQC
        calc(6, &mut database, &opers, 4, &mut err); // SLC (might sleep for 10 seconds)

        assert_eq!(database[5], 30); // EQC: E1 = C1 = 30
        assert_eq!(database[6], 10); // SLC: F1 = A1 = 10
    }

    #[test]
    fn test_calc_value_combinations() {
        let mut database = vec![0, 10, 5, 0, 0, 0, 0]; // Index 0 unused
        let mut err = vec![false; 7];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            }, // B1 = 5
            Ops {
                opcpde: String::from("VCA"),
                cell1: 7,
                cell2: 1,
            }, // C1 = 7 + A1 = 17
            Ops {
                opcpde: String::from("CVA"),
                cell1: 2,
                cell2: 8,
            }, // D1 = B1 + 8 = 13
            Ops {
                opcpde: String::from("VCS"),
                cell1: 15,
                cell2: 2,
            }, // E1 = 15 - B1 = 10
            Ops {
                opcpde: String::from("VCD"),
                cell1: 100,
                cell2: 1,
            }, // F1 = 100 / A1 = 10
        ];

        for i in 3..=6 {
            calc(i, &mut database, &opers, 3, &mut err);
        }

        assert_eq!(database[3], 17); // VCA: 7 + A1 = 7 + 10 = 17
        assert_eq!(database[4], 13); // CVA: B1 + 8 = 5 + 8 = 13
        assert_eq!(database[5], 10); // VCS: 15 - B1 = 15 - 5 = 10
        assert_eq!(database[6], 10); // VCD: 100 / A1 = 100 / 10 = 10
    }

    #[test]
    fn test_calc_statistical_functions() {
        // Set up a row of cells with values 10, 20, 30, 40, 50
        let mut database = vec![0, 10, 20, 30, 40, 50, 0, 0, 0, 0, 0]; // Index 0 unused
        let mut err = vec![false; 11];
        let len_h = 5; // Width of 5 cells

        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 20,
                cell2: -1,
            }, // B1 = 20
            Ops {
                opcpde: String::from("EQV"),
                cell1: 30,
                cell2: -1,
            }, // C1 = 30
            Ops {
                opcpde: String::from("EQV"),
                cell1: 40,
                cell2: -1,
            }, // D1 = 40
            Ops {
                opcpde: String::from("EQV"),
                cell1: 50,
                cell2: -1,
            }, // E1 = 50
            Ops {
                opcpde: String::from("MIN"),
                cell1: 1,
                cell2: 5,
            }, // F1 = MIN(A1:E1) = 10
            Ops {
                opcpde: String::from("MAX"),
                cell1: 1,
                cell2: 5,
            }, // G1 = MAX(A1:E1) = 50
            Ops {
                opcpde: String::from("SUM"),
                cell1: 1,
                cell2: 5,
            }, // H1 = SUM(A1:E1) = 150
            Ops {
                opcpde: String::from("MEA"),
                cell1: 1,
                cell2: 5,
            }, // I1 = MEA(A1:E1) = 30
            Ops {
                opcpde: String::from("STD"),
                cell1: 1,
                cell2: 5,
            }, // J1 = STD(A1:E1)
        ];

        // Calculate statistical operations
        for i in 6..=10 {
            calc(i, &mut database, &opers, len_h, &mut err);
        }

        assert_eq!(database[6], 10); // MIN(A1:E1) = 10
        assert_eq!(database[7], 50); // MAX(A1:E1) = 50
        assert_eq!(database[8], 150); // SUM(A1:E1) = 150
        assert_eq!(database[9], 30); // MEA(A1:E1) = 30

        // STD calculation should be approximately √((10-30)²+(20-30)²+(30-30)²+(40-30)²+(50-30)²)/5 = √500/5 ≈ 14.14
        let expected_std = ((400.0 + 100.0 + 0.0 + 100.0 + 400.0) / 5.0_f32).sqrt() as i32;
        assert_eq!(database[10], expected_std); // STD(A1:E1) ≈ 14.14 -> 15 (rounded)
    }

    #[test]
    fn test_sleep_operations() {
        let mut database = vec![0, 0, 0];
        let mut err = vec![false; 3];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("SLV"),
                cell1: 0,
                cell2: -1,
            }, // A1 = Sleep 0s, value 0
            Ops {
                opcpde: String::from("SLV"),
                cell1: 1,
                cell2: -1,
            }, // B1 = Sleep 1s, value 1
        ];

        // Use a timer to verify it sleeps
        let start = std::time::Instant::now();
        calc(1, &mut database, &opers, 2, &mut err);
        let elapsed_a1 = start.elapsed();

        let start = std::time::Instant::now();
        calc(2, &mut database, &opers, 2, &mut err);
        let elapsed_b1 = start.elapsed();

        assert_eq!(database[1], 0);
        assert_eq!(database[2], 1);
        assert!(elapsed_a1.as_millis() < 100); // A1 should execute quickly
        assert!(elapsed_b1.as_millis() >= 900); // B1 should sleep ~1 second
    }

    #[test]
    fn test_error_handling_in_operations() {
        let mut database = vec![0, 10, 0, 0, 0, 0];
        let mut err = vec![false, false, false, false, false, false];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 0,
                cell2: -1,
            }, // B1 = 0
            Ops {
                opcpde: String::from("CCD"),
                cell1: 1,
                cell2: 2,
            }, // C1 = A1 / B1 = 10 / 0 (error)
            Ops {
                opcpde: String::from("VVD"),
                cell1: 20,
                cell2: 0,
            }, // D1 = 20 / 0 (error)
            Ops {
                opcpde: String::from("CVA"),
                cell1: 3,
                cell2: 5,
            }, // E1 = C1 + 5 (propagated error)
        ];

        for i in 3..=5 {
            calc(i, &mut database, &opers, 3, &mut err);
        }

        assert!(err[3]); // C1 has error (division by zero)
        assert!(err[4]); // D1 has error (direct division by zero)
        assert!(err[5]); // E1 has error (derived from C1's error)
    }

    #[test]
    fn test_val_update_complex_dependencies() {
        // Testing a more complex dependency chain: A1 -> B1 -> C1 -> D1
        let mut database = vec![0, 0, 0, 0, 0]; // Index 0 unused, cells 1-4
        let mut err = vec![false, false, false, false, false];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            }, // A1 = 5
            Ops {
                opcpde: String::from("CVM"),
                cell1: 1,
                cell2: 2,
            }, // B1 = A1 * 2 = 10
            Ops {
                opcpde: String::from("CVA"),
                cell1: 2,
                cell2: 5,
            }, // C1 = B1 + 5 = 15
            Ops {
                opcpde: String::from("CCM"),
                cell1: 3,
                cell2: 1,
            }, // D1 = C1 * A1 = 15 * 5 = 75
        ];

        // Topo order: 1, 2, 3, 4 (A1, B1, C1, D1)
        let topo_arr = vec![4, 1, 2, 3, 4]; // First element is count, then indices in order

        val_update(&topo_arr, &mut database, &opers, 4, &mut err);

        assert_eq!(database[1], 5); // A1 = 5
        assert_eq!(database[2], 10); // B1 = 5 * 2 = 10
        assert_eq!(database[3], 15); // C1 = 10 + 5 = 15
        assert_eq!(database[4], 75); // D1 = 15 * 5 = 75
    }

    #[test]
    fn test_error_propagation() {
        let mut database = vec![0, 0, 0, 0];
        let mut err = vec![false, true, false, false]; // A1 has an error
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("CCA"),
                cell1: 1,
                cell2: 2,
            }, // C1 = A1 + B1, A1 has error
        ];

        calc(3, &mut database, &opers, 3, &mut err);
        assert!(err[3]); // Error propagates
    }

    #[test]
    fn test_division_by_zero() {
        let mut database = vec![0, 10, 0, 0]; // A1=10, B1=0
        let mut err = vec![false, false, false, false];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("EQV"),
                cell1: 0,
                cell2: -1,
            },
            Ops {
                opcpde: String::from("CCD"),
                cell1: 1,
                cell2: 2,
            }, // C1 = A1 / B1
        ];

        calc(3, &mut database, &opers, 3, &mut err);
        assert!(err[3]); // Division by zero causes error
    }

    #[test]
    fn test_val_update() {
        let mut database = vec![0, 0, 0, 0, 0]; // Index 0 unused, cells 1-4
        let mut err = vec![false, false, false, false, false];
        let opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            }, // Unused
            Ops {
                opcpde: String::from("EQV"),
                cell1: 10,
                cell2: -1,
            }, // A1 = 10
            Ops {
                opcpde: String::from("EQV"),
                cell1: 5,
                cell2: -1,
            }, // B1 = 5
            Ops {
                opcpde: String::from("CCA"),
                cell1: 1,
                cell2: 2,
            }, // C1 = A1 + B1
            Ops {
                opcpde: String::from("CCM"),
                cell1: 3,
                cell2: 1,
            }, // D1 = C1 * A1
        ];

        // Topo order: 1, 2, 3, 4 (A1, B1, C1, D1)
        let topo_arr = vec![4, 1, 2, 3, 4]; // First element is count, then indices in order

        val_update(&topo_arr, &mut database, &opers, 4, &mut err);

        assert_eq!(database[1], 10); // A1 = 10
        assert_eq!(database[2], 5); // B1 = 5
        assert_eq!(database[3], 15); // C1 = 10 + 5 = 15
        assert_eq!(database[4], 150); // D1 = 15 * 10 = 150
    }

    #[test]
    fn test_cell_update_simple() {
        let mut database = vec![0, 0, 0, 0];
        let mut err = vec![false, false, false, false];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
        ];
        let mut sensi = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut indegree = vec![0, 0, 0, 0];

        // Set A1 to 10
        let inp_arr = vec![
            String::from("A1"),  // Cell
            String::from("EQV"), // Operation
            String::from("10"),  // Value 1
            String::from("0"),   // Value 2
        ];

        let result = cell_update(
            &inp_arr,
            &mut database,
            &mut sensi,
            &mut opers,
            2,
            &mut indegree,
            &mut err,
        );

        assert_eq!(result, 1); // Update successful
        assert_eq!(database[1], 10); // A1 = 10
        assert!(!err[1]); // No error
    }

    #[test]
    fn test_cell_update_with_dependencies() {
        let mut database = vec![0, 0, 0, 0];
        let mut err = vec![false, false, false, false];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
        ];
        let mut sensi = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut indegree = vec![0, 0, 0, 0];

        // Set A1 to 10
        let inp_arr1 = vec![
            String::from("A1"),
            String::from("EQV"),
            String::from("10"),
            String::from("0"),
        ];

        // Set B1 to 5
        let inp_arr2 = vec![
            String::from("B1"),
            String::from("EQV"),
            String::from("5"),
            String::from("0"),
        ];

        // Set C1 to A1 + B1
        let inp_arr3 = vec![
            String::from("C1"),
            String::from("CCA"),
            String::from("A1"),
            String::from("B1"),
        ];

        cell_update(
            &inp_arr1,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );
        cell_update(
            &inp_arr2,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );
        let result = cell_update(
            &inp_arr3,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );

        assert_eq!(result, 1); // Update successful
        assert_eq!(database[3], 15); // C1 = A1 + B1 = 10 + 5 = 15

        // Now update A1 and check if C1 updates
        let inp_arr4 = vec![
            String::from("A1"),
            String::from("EQV"),
            String::from("20"),
            String::from("0"),
        ];

        cell_update(
            &inp_arr4,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );
        assert_eq!(database[1], 20); // A1 = 20
        assert_eq!(database[3], 25); // C1 = A1 + B1 = 20 + 5 = 25
    }

    #[test]
    fn test_cell_update_cycle_detection() {
        let mut database = vec![0, 0, 0, 0];
        let mut err = vec![false, false, false, false];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1,
            },
        ];
        let mut sensi = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut indegree = vec![0, 0, 0, 0];

        // Set A1 to B1 + 1
        let inp_arr1 = vec![
            String::from("A1"),
            String::from("CVA"),
            String::from("B1"),
            String::from("1"),
        ];

        // Set B1 to A1 + 1 (creates cycle)
        let inp_arr2 = vec![
            String::from("B1"),
            String::from("CVA"),
            String::from("A1"),
            String::from("1"),
        ];

        let result1 = cell_update(
            &inp_arr1,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );
        let result2 = cell_update(
            &inp_arr2,
            &mut database,
            &mut sensi,
            &mut opers,
            3,
            &mut indegree,
            &mut err,
        );

        assert_eq!(result1, 1); // First update is fine
        assert_eq!(result2, 0); // Second update creates cycle, should return 0
    }

    #[test]
    fn test_range_operations() {
        let mut database = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // Cells 1-9 with values 1-9
        let mut err = vec![false; 10];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1
            };
            10
        ];
        let mut sensi = vec![Vec::new(); 10];
        let mut indegree = vec![0; 10];

        // Initialize cells with values
        for i in 1..9 {
            let inp_arr = vec![
                format!("A{}", i),
                String::from("EQV"),
                format!("{}", i),
                String::from("0"),
            ];
            cell_update(
                &inp_arr,
                &mut database,
                &mut sensi,
                &mut opers,
                1,
                &mut indegree,
                &mut err,
            );
        }

        // Set A9 to SUM of range A1:A8
        let inp_arr = vec![
            String::from("A9"),
            String::from("SUM"),
            String::from("A1"),
            String::from("A8"),
        ];

        let result = cell_update(
            &inp_arr,
            &mut database,
            &mut sensi,
            &mut opers,
            1,
            &mut indegree,
            &mut err,
        );

        assert_eq!(result, 1); // Update successful
        assert_eq!(database[9], 36);

        // Change A1 and check if A9 updates
        let inp_arr_update = vec![
            String::from("A1"),
            String::from("EQV"),
            String::from("10"),
            String::from("0"),
        ];

        cell_update(
            &inp_arr_update,
            &mut database,
            &mut sensi,
            &mut opers,
            1,
            &mut indegree,
            &mut err,
        );
        assert_eq!(database[1], 10); // A1 = 10
        assert_eq!(database[9], 45);

        // Update A9 to sum only A1:A5 instead of A1:A8
        let inp_arr_range_update = vec![
            String::from("A9"),
            String::from("SUM"),
            String::from("A1"),
            String::from("A5"),
        ];

        cell_update(
            &inp_arr_range_update,
            &mut database,
            &mut sensi,
            &mut opers,
            1,
            &mut indegree,
            &mut err,
        );
        assert_eq!(database[9], 24); // Sum of (10+2+3+4+5) = 24

        // Make sure updating a cell outside the new range doesn't affect the sum
        let inp_arr_out_of_range = vec![
            String::from("A8"),
            String::from("EQV"),
            String::from("100"),
            String::from("0"),
        ];

        cell_update(
            &inp_arr_out_of_range,
            &mut database,
            &mut sensi,
            &mut opers,
            1,
            &mut indegree,
            &mut err,
        );
        assert_eq!(database[8], 100); // A8 = 100
        assert_eq!(database[9], 24); // Sum remains unchanged as A8 is outside the range
    }

    #[test]
    fn test_complex_cell_updates() {
        let len_h = 10;
        let len_v = 10;
        let mut database = vec![0; (len_h * len_v + 1) as usize];
        let mut err = vec![false; (len_h * len_v + 1) as usize];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1
            };
            (len_h * len_v + 1) as usize
        ];
        let mut indegree = vec![0; (len_h * len_v + 1) as usize];
        let mut sensi = vec![Vec::<i32>::new(); (len_h * len_v + 1) as usize];

        let mut status;

        // Create a series of complex updates to test the spreadsheet functionality
        let test_inputs = [
            "A1=SUM(B1:B4)",
            "A1=MIN(B2:B8)",
            "A1=1",
            "A1=MAX(B2:B8)",
            "A1=B2",
        ];

        // Process each test input
        for (i, input) in test_inputs.iter().enumerate() {
            println!("Processing input {}: {}", i + 1, input);

            let input = input.trim_end().to_string();
            // rest of the existing code to process the input

            let out = utils::input::input(&input, len_h, len_v);
            status = out[4].clone();
            if status == "ok" {
                cell_update(
                    &out,
                    &mut database,
                    &mut sensi,
                    &mut opers,
                    len_h,
                    &mut indegree,
                    &mut err,
                );
            }
        }
        assert_eq!(database[1], 0); // A1 = 0
    }

    #[test]
    fn test_complex_cell_updates_cyclic() {
        let len_h = 10;
        let len_v = 10;
        let mut database = vec![0; (len_h * len_v + 1) as usize];
        let mut err = vec![false; (len_h * len_v + 1) as usize];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1
            };
            (len_h * len_v + 1) as usize
        ];
        let mut indegree = vec![0; (len_h * len_v + 1) as usize];
        let mut sensi = vec![Vec::<i32>::new(); (len_h * len_v + 1) as usize];

        let mut suc = 0;
        let mut status;

        // Create a series of complex updates to test the spreadsheet functionality
        let test_inputs = ["A1=A2", "A1=MAX(B2:B8)", "A1=A2", "A1=MIN(B2:B8)", "A1=A1"];

        // Process each test input
        for (i, input) in test_inputs.iter().enumerate() {
            println!("Processing input {}: {}", i + 1, input);

            let input = input.trim_end().to_string();
            // rest of the existing code to process the input

            let out = utils::input::input(&input, len_h, len_v);
            status = out[4].clone();
            if status == "ok" {
                suc = cell_update(
                    &out,
                    &mut database,
                    &mut sensi,
                    &mut opers,
                    len_h,
                    &mut indegree,
                    &mut err,
                );
            }
        }
        assert!(suc == 0);
    }

    #[test]
    fn test_complex_range_updates_cyclic() {
        let len_h = 10;
        let len_v = 10;
        let mut database = vec![0; (len_h * len_v + 1) as usize];
        let mut err = vec![false; (len_h * len_v + 1) as usize];
        let mut opers = vec![
            Ops {
                opcpde: String::new(),
                cell1: -1,
                cell2: -1
            };
            (len_h * len_v + 1) as usize
        ];
        let mut indegree = vec![0; (len_h * len_v + 1) as usize];
        let mut sensi = vec![Vec::<i32>::new(); (len_h * len_v + 1) as usize];

        let mut suc = 0;
        let mut status;

        // Create a series of complex updates to test the spreadsheet functionality
        let test_inputs = ["A1=MAX(B2:B8)", "A1=MAX(A1:B5)"];

        // Process each test input
        for (i, input) in test_inputs.iter().enumerate() {
            println!("Processing input {}: {}", i + 1, input);

            let input = input.trim_end().to_string();
            // rest of the existing code to process the input

            let out = utils::input::input(&input, len_h, len_v);
            status = out[4].clone();
            if status == "ok" {
                suc = cell_update(
                    &out,
                    &mut database,
                    &mut sensi,
                    &mut opers,
                    len_h,
                    &mut indegree,
                    &mut err,
                );
            }
        }
        assert!(suc == 0);
    }
}
