use regex::Regex;


fn is_integer(inp: &String) -> bool {
    let number = Regex::new(r"^-?\d+$").unwrap(); // regex for integer with optional unary minus
    number.is_match(inp)
}

fn cell_to_int(a: &str) -> i32{
    let mut col = 0;
    let b = a.chars();
    let mut part = 0;
    for c in b.clone(){
        if c.is_alphabetic() {
            part += 1;
        } else {
            break;
        }
    }

    for i in a[..part].chars() {
        let diff = i as i32 - 'A' as i32 + 1;
        
        
        if 1<=diff && diff<=26 {
            col *= 26;
            col += diff;
        } else {
            
            break;
        }
    }
    
    let row: i32 = a[part..].parse().unwrap_or(0);

    col * 1000 + row
}




fn is_valid_cell(cell: &str, n_rows:i32, n_cols:i32) -> bool {
    let cell = cell_to_int(cell);
    let row = cell % 1000;
    let col = cell / 1000;
    // println!("{cell} {row} {col}");
    // println!("{} {} {}",row,col,cell);
    // Check if the row and column are within the valid range
    if row>=1 && row<=n_rows && col>=1 && col<=n_cols {
        return true;
    }
    false

}
fn is_valid_range(cell1 : &String, cell2 : &String, n_rows:i32, n_cols:i32) -> bool {
    let cell1 = cell_to_int(cell1);
    let cell2 = cell_to_int(cell2);
    let row1 = cell1 % 1000;
    let row2 = cell2 % 1000;
    let col1 = cell1 / 1000;
    let col2 = cell2 / 1000;

    if row1>=1 && row1<=n_rows && col1>=1 && col1<=n_cols && row2>=1 && row2<=n_rows && col2>=1 && col2<=n_cols {
        
        if row1<=row2 && col1<=col2{
            return true;
        }
        else{
            return false;
        }
    }
    return false;
}

pub fn input(inp: &String,n_cols:i32,n_rows:i32) -> Vec<String> {

    // () used in regex for capture
    let mut output: Vec<String>=Vec::new();
    let arithimetic = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)\s*([+\-*/])\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)$").unwrap(); //regex for arth op
    let rangeop = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(MIN|MAX|AVG|SUM|STDEV)\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*:\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*\)$").unwrap(); //regex for range operation
    let assignment = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?\d+|[A-Z]{1,3}[1-9][0-9]{0,2})$").unwrap(); //regex for assignment
    let sleep = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*SLEEP\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2}|\d+)\s*\)$").unwrap();

    //regex for sleep
    let scroll_to = Regex::new(r"^scroll_to\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*\)$").unwrap(); //regex for scroll_to

    if let Some(caps) = arithimetic.captures(inp) {
        let target_cell = caps[1].to_string();
        let cell1  = caps[2].to_string();
        let cell2  = caps[4].to_string();
        let op = caps[3].to_string();
        output.push(target_cell.clone());
        output.push(String::new());
        if is_integer(&cell1){
            output[1].push_str("V");
        }
        else{
            output[1].push_str("C");
        }
        if is_integer(&cell2){
            output[1].push_str("V");
        }
        else{
            output[1].push_str("C");
        }

        if op=="+"{
            output[1].push_str("A");
        }
        else if op=="-"{
            output[1].push_str("S");
        }
        else if op=="*"{
            output[1].push_str("M");
        }
        else if op=="/"{
            output[1].push_str("D");
        }
        output.push(cell1.clone());
        output.push(cell2.clone());
        if is_valid_cell(&target_cell, n_rows, n_cols) && (is_integer(&cell1) || is_valid_cell(&cell1, n_rows, n_cols)) && (is_integer(&cell2) || is_valid_cell(&cell2, n_rows, n_cols)){
            output.push(String::from("ok"));
        }
        else{
            output.push(String::from("Cell Out Of Bounds"));
        }
    }

    else if let Some(caps) = rangeop.captures(inp) {
        let target_cell = caps[1].to_string();
        let mut operation = caps[2].to_string();
        if operation=="STDEV"{
            operation=String::from("STD");
        }
        if operation=="AVG"{
            operation=String::from("MEA");
        }
        let cell1  = caps[3].to_string();
        let cell2  = caps[4].to_string();
        output.push(target_cell.clone());
        output.push(operation);
        output.push(cell1.clone());
        output.push(cell2.clone());

        if is_valid_cell(&target_cell, n_rows, n_cols) && is_valid_range(&cell1, &cell2, n_rows, n_cols){
            output.push(String::from("ok"));
        }
        else{
            if is_valid_cell(&target_cell, n_rows, n_cols){
                // println!("{}",&target_cell);
                output.push(String::from("Invalid Range"));
            }
            else{
                // println!("{}",&target_cell);
                output.push(String::from("Cell Out Of Bounds"));
            }
        }
    }

    else if let Some(caps) = assignment.captures(inp) {
        let target_cell = caps[1].to_string();
        let cell1  = caps[2].to_string();
        output.push(target_cell.clone());
        output.push(String::new());
        if is_integer(&cell1){
            output[1].push_str("EQV");
        }
        else{
            output[1].push_str("EQC");
        }
        output.push(cell1.clone());
        output.push(String::new());
        if is_integer(&cell1){
            if is_valid_cell(&target_cell, n_rows, n_cols){
                output.push(String::from("ok"));
            }
            else{
                output.push(String::from("Cell Out Of Bounds"));
            }

        }
        else{
            if is_valid_cell(&target_cell, n_rows, n_cols) && is_valid_cell(&cell1, n_rows, n_cols){
                output.push(String::from("ok"));
            }
            else{
                output.push(String::from("Cell Out Of Bounds"));
            }
        }

    }

    else if let Some(caps) = sleep.captures(inp) {
        let target_cell  = caps[1].to_string();
        let cell1  = caps[2].to_string();
        output.push(target_cell.clone());
        output.push(String::from("SL"));
        if is_integer(&cell1){
            output[1].push_str("V");
        }
        else{
            output[1].push_str("C");
        }
        output.push(cell1.clone());
        output.push(String::new());

        if is_integer(&cell1){
            if is_valid_cell(&target_cell, n_rows, n_cols){
                output.push(String::from("ok"));
            }
            else{
                output.push(String::from("Cell Out Of Bounds"));
            }

        }
        else{
            if is_valid_cell(&target_cell, n_rows, n_cols) && is_valid_cell(&cell1, n_rows, n_cols){
                output.push(String::from("ok"));
            }
            else{
                output.push(String::from("Cell Out Of Bounds"));
            }
        }
    }

    else if let Some(caps) = scroll_to.captures(inp) {
        let cell1  = caps[1].to_string();
        output.push(cell1.clone());
        output.push(String::from("SRL"));
        output.push(String::new());
        output.push(String::new());
        if is_valid_cell(&cell1, n_rows, n_cols){
            output.push(String::from("ok"));
        }
        else{
            output.push(String::from("Cell Out Of Bounds"));
        }
    }

    else{
        output.push(String::new());
        output.push(String::new());
        output.push(String::new());
        output.push(String::new());
        output.push(String::from("Invalid Input"));
    }
    return output;
}


// pub fn main2(){
//     let s =String::from( "A3 = A4 + 5");
//     let outp = input(&s,10,10);
//     println!("{:?}",outp)
// }