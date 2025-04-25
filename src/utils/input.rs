//! This module contains functions for parsing input and checking if input is valid.
use crate::cell_to_int;

/// Checks if the input is of arithmetic type.
/// 
/// # Arguments
/// * `input` - A string slice containing the input to check
/// 
/// # Returns
/// * `bool` - true if input is arithmetic (does not contain parentheses), false otherwise
fn is_arth(input: &str) -> bool {
    for c in input.chars() {
        if c == '(' {
            return false;
        }
    }
    true
}

/// Checks if the input is a scroll operation.
/// 
/// # Arguments
/// * `input` - A string slice containing the input to check
/// 
/// # Returns
/// * `bool` - true if input is a scroll operation (no '=' character), false otherwise
fn is_scroll(input: &str) -> bool {
    // if input is found true by is_arth and it does not contain =, then it is scroll_to
    for c in input.chars() {
        if c == '=' {
            return false;
        }
    }
    true
}

/// Checks if the input string represents an integer.
/// 
/// # Arguments
/// * `input` - A string slice containing the input to check
/// 
/// # Returns
/// * `bool` - true if input is an integer value, false otherwise
fn is_integer(input: &str) -> bool {
    let mut first = 1;
    for c in input.chars() {
        if first == 1 {
            if c == '-' || c == '+' {
                continue;
            }
            first = 0;
        }

        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

/// Validates if a cell reference is within bounds.
/// 
/// # Arguments
/// * `cell` - A string slice containing the cell reference (e.g., "A1")
/// * `len_h` - An i32 representing the horizontal boundary (columns)
/// * `len_v` - An i32 representing the vertical boundary (rows)
/// 
/// # Returns
/// * `bool` - true if the cell is valid and within bounds, false otherwise
pub fn is_valid_cell(cell: &str, len_h: i32, len_v: i32) -> bool {
    // input no of rows,no of cols
    let n = cell.len();
    if n < 2 {
        return false;
    }
    let mut first = 1;
    let mut state = 0;
    for i in cell.chars() {
        if first == 1 {
            first = 0;
            if !i.is_ascii_uppercase() {
                return false;
            }
            continue;
        }

        if state == 0 {
            if !i.is_ascii_uppercase() {
                state = 1;
            }
        } else if !i.is_ascii_digit() {
            return false;
        }
    }
    if state == 0 {
        return false;
    }
    let k = cell_to_int(cell);
    let r = k % 1000;
    let c = k / 1000;
    if r <= len_v && c <= len_h && r > 0 && c > 0 {
        return true;
    }
    false
}

/// Validates if a cell range is valid and within bounds.
/// 
/// # Arguments
/// * `cell1` - A string slice containing the first cell reference
/// * `cell2` - A string slice containing the second cell reference
/// * `len_h` - An i32 representing the horizontal boundary (columns)
/// * `len_v` - An i32 representing the vertical boundary (rows)
/// 
/// # Returns
/// * `bool` - true if the range is valid and within bounds, false otherwise
fn is_valid_range(cell1: &str, cell2: &str, len_h: i32, len_v: i32) -> bool {
    let k1 = cell_to_int(cell1);
    let r1 = k1 % 1000;
    let c1 = k1 / 1000;
    let k2 = cell_to_int(cell2);
    let r2 = k2 % 1000;
    let c2 = k2 / 1000;

    !(r1 > r2 || c1 > c2) && (r1 <= len_v && c1 <= len_h) && (r2 <= len_v && c2 <= len_h) &&
        (r1 > 0 && c1 > 0) && (r2 > 0 && c2 > 0)
}

/// Checks for errors in the parsed input based on operation type and cell references.
/// 
/// # Arguments
/// * `input` - A string slice containing the original input
/// * `output` - A slice of Strings containing the parsed components
/// * `len_h` - An i32 representing the horizontal boundary (columns)
/// * `len_v` - An i32 representing the vertical boundary (rows)
/// 
/// # Returns
/// * `String` - "ok" if no errors, otherwise a relevant error message
fn check_err(input: &str, output: &[String], len_h: i32, len_v: i32) -> String {
    let mut message = String::from("ok");
    let vec1 = ["MEA", "STD", "SUM", "MIN", "MAX"];
    let vec2 = [
        "VVA", "CVA", "VCA", "CCA", "VVS", "CVS", "VCS", "CCS", "VVM", "CVM", "VCM", "CCM", "VVD",
        "CVD", "VCD", "CCD",
    ];
    if output[1].len() != 3 {
        message = String::from("Invalid Operation");
        return message;
    }
    if output[1] == "SRL" {
        let mut temp = String::new();
        for i in input.chars() {
            if i == ' ' {
                break;
            }
            temp.push(i);
        }
        if temp != "scroll_to" {
            message = String::from("Invalid Operation");
        } else if !is_valid_cell(&output[0], len_h, len_v) {
            message = String::from("Scroll Cell out of bounds");
        }
    } else {
        if !is_valid_cell(&output[0], len_h, len_v) {
            message = String::from("Assigned Cell out of bounds");
            return message;
        }

        if output[1] == "SLC" || output[1] == "EQC" {
            if !is_valid_cell(&output[2], len_h, len_v) {
                message = String::from("Invalid Cell");
                return message;
            }
        } else if output[1] == "SLV" || output[1] == "EQV" {
            return message;
        } else if vec1.contains(&(output[1].as_str())) {
            if !is_valid_range(&output[2], &output[3], len_h, len_v) {
                message = String::from("Invalid Range");
                return message;
            }
            return message;
        } else if vec2.contains(&(output[1].as_str())) {
            let f = output[1].chars().next().unwrap();
            let s = output[1].chars().nth(1).unwrap();
            if f == 'C' {
                if !is_valid_cell(&output[2], len_h, len_v) {
                    message = String::from("Invalid Cell");
                    return message;
                }
                return message;
            }

            if s == 'C' {
                if !is_valid_cell(&output[3], len_h, len_v) {
                    message = String::from("Invalid Cell");
                    return message;
                }
                return message;
            }
        } else {
            message = String::from("Invalid Operation");
            return message;
        }
    }
    message
}

/// Parses input into components without validation.
/// 
/// # OPCODES
/// Strings of length 3 to determine type of operation:
/// 
/// ## Arithmetic Operations
/// Format: [Operand1Type][Operand2Type][OperationType]
/// - Operand types:
///   - 'C': Cell reference (e.g., 'A1')
///   - 'V': Value (integer)
/// - Operation types:
///   - 'A': Addition (+)
///   - 'M': Multiplication (*)
///   - 'D': Division (/)
///   - 'S': Subtraction (-)
/// - Examples:
///   - "VVA": Value + Value
///   - "CCA": Cell + Cell
///   - "CVD": Cell / Value
/// 
/// ## Assignment Operations
/// - "EQV": Assign value to cell (e.g., A1=5)
/// - "EQC": Assign cell value to another cell (e.g., A1=B2)
/// 
/// ## Function Operations
/// - "MEA": Average function (AVG)
/// - "STD": Standard deviation function (STDEV)
/// - "SUM": Sum function
/// - "MIN": Minimum value function
/// - "MAX": Maximum value function
/// 
/// ## Special Operations
/// - "SRL": Scroll to a specific cell
/// - "SLV": Sleep for a value (time in ms)
/// - "SLC": Sleep for a cell value (time in ms)
/// 
/// # Arguments
/// * `input` - A string slice containing the input to parse
/// 
/// # Returns
/// * `Vec<String>` - Vector containing the parsed components:
///   - `output[0]` - Destination Cell
///   - `output[1]` - OPCODE (as described above)
///   - `output[2]` - First operand
///   - `output[3]` - Second operand (may be empty)
pub fn help_input(input: &str) -> Vec<String> {
    let mut output = vec![String::new(); 4];
    let input_arr: Vec<char> = input.chars().collect();
    let n = input_arr.len();
    if is_scroll(input) {
        let mut i = 0;
        output[1] = String::from("SRL");
        while i < n && input_arr[i] != ' ' {
            i += 1;
        }
        // put the cell in output[0]- target cell
        i += 1;
        while i < n {
            output[0].push(input_arr[i]);
            i += 1;
        }
        return output;
    }
    let mut i = 0;

    while i < n && input_arr[i] != '=' {
        output[0].push(input_arr[i]);
        i += 1;
    }

    if is_arth(input) {
        i += 1;
        while i < n && input_arr[i] == ' ' {
            i += 1;
        }
        output[2].push(input_arr[i]);
        i += 1;
        let mut oper;
        if i == n {
            output[1].push('E');
            output[1].push('Q');
            if is_integer(&output[2]) {
                output[1].push('V');
            } else {
                output[1].push('C');
            }
            return output;
        }
        while i < n
            && (input_arr[i] != '*'
                && input_arr[i] != '/'
                && input_arr[i] != '+'
                && input_arr[i] != '-')
        {
            output[2].push(input_arr[i]);
            i += 1;
            if i == n {
                output[1].push('E');
                output[1].push('Q');
                if is_integer(&output[2]) {
                    output[1].push('V');
                } else {
                    output[1].push('C');
                }
                return output;
            }
        }

        oper = input_arr[i];
        if oper == '+' {
            oper = 'A';
        } else if oper == '-' {
            oper = 'S';
        } else if oper == '*' {
            oper = 'M';
        } else if oper == '/' {
            oper = 'D';
        }
        i += 1;
        while input_arr[i] == ' ' {
            i += 1;
        }
        while i < n {
            output[3].push(input_arr[i]);
            i += 1;
        }

        if is_integer(&output[2]) {
            output[1].push('V');
        } else {
            output[1].push('C');
        }

        if is_integer(&output[3]) {
            output[1].push('V');
        } else {
            output[1].push('C');
        }

        output[1].push(oper);
    } else {
        i += 1;
        while i < n && input_arr[i] == ' ' {
            i += 1;
        }
        while i < n && input_arr[i] != '(' {
            output[1].push(input_arr[i]);
            i += 1;
        }
        i += 1;
        if output[1] == *"SLEEP" {
            output[1] = String::from("SL");
            while i < n && input_arr[i] != ')' {
                output[2].push(input_arr[i]);
                i += 1;
            }
        } else {
            while i < n && input_arr[i] != ':' {
                output[2].push(input_arr[i]);
                i += 1;
            }
            i += 1;
            while i < n && input_arr[i] != ')' {
                output[3].push(input_arr[i]);
                i += 1;
            }
        }
    }

    if output[1] == *"STDEV" {
        output[1] = String::from("STD");
    } else if output[1] == *"AVG" {
        output[1] = String::from("MEA");
    } else if output[1] == *"SL" {
        if is_integer(&output[2]) {
            output[1].push('V');
        } else {
            output[1].push('C');
        }
    }

    output
}

/// Parses and validates input for spreadsheet operations.
/// 
/// # Arguments
/// * `input` - A string slice containing the input to parse and validate
/// * `len_h` - An i32 representing the horizontal boundary (columns)
/// * `len_v` - An i32 representing the vertical boundary (rows)
/// 
/// # Returns
/// * `Vec<String>` - Vector containing the parsed components(output of `help_input` function) and validation message (output of `check_err` function).
pub fn input(input: &str, len_h: i32, len_v: i32) -> Vec<String> {
    let mut output = help_input(input);

    let message = check_err(input, &output, len_h, len_v);
    output.push(message);

    output
}
// fn main(){
//     // let outp=is_valid_cell(&String::from("SUM"),100,100);
//     // println!("{}",outp);
//     let inp = String::from("A1=SUM");
//     let output = input(&inp,55,55);
//     for i in 0..5{
//         println!("{}",output[i]);
//     }
// }