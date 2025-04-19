use regex::Regex;


use once_cell::sync::Lazy;


static ARITHMETIC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)\s*([+\-*/])\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)$").unwrap()
});

static RANGEOP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(MIN|MAX|AVG|SUM|STDEV)\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*:\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*\)$").unwrap()
});

static ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?\d+|[A-Z]{1,3}[1-9][0-9]{0,2})$").unwrap()
});

static SLEEP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*SLEEP\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2}|\d+)\s*\)$").unwrap()
});

// Same for assignment, sleep, scroll_to
static SCROLL_TO: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^scroll_to\s+([A-Z]{1,3}[1-9][0-9]{0,2})$").unwrap()
});

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
        
        return row1<=row2 && col1<=col2
    }
    false
}

pub fn input(inp: &String,n_cols:i32,n_rows:i32) -> Vec<String> {

    // () used in regex for capture
    let mut output: Vec<String>=Vec::new();


    if let Some(caps) = ARITHMETIC.captures(inp) {
        let target_cell = caps[1].to_string();
        let cell1  = caps[2].to_string();
        let cell2  = caps[4].to_string();
        let op = caps[3].to_string();
        output.push(target_cell.clone());
        output.push(String::new());
        if is_integer(&cell1){
            output[1].push('V');
        }
        else{
            output[1].push('C');
        }
        if is_integer(&cell2){
            output[1].push('V');
        }
        else{
            output[1].push('C');
        }

        if op=="+"{
            output[1].push('A');
        }
        else if op=="-"{
            output[1].push('S');
        }
        else if op=="*"{
            output[1].push('M');
        }
        else if op=="/"{
            output[1].push('D');
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

    else if let Some(caps) = RANGEOP.captures(inp) {
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
        else if is_valid_cell(&target_cell, n_rows, n_cols){
            // println!("{}",&target_cell);
            output.push(String::from("Invalid Range"));
        }
        else{
            // println!("{}",&target_cell);
            output.push(String::from("Cell Out Of Bounds"));
        }
    }

    else if let Some(caps) = ASSIGNMENT.captures(inp) {
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
        else if is_valid_cell(&target_cell, n_rows, n_cols) && is_valid_cell(&cell1, n_rows, n_cols){
            output.push(String::from("ok"));
        }
        else{
            output.push(String::from("Cell Out Of Bounds"));
        }

    }

    else if let Some(caps) = SLEEP.captures(inp) {
        let target_cell  = caps[1].to_string();
        let cell1  = caps[2].to_string();
        output.push(target_cell.clone());
        output.push(String::from("SL"));
        if is_integer(&cell1){
            output[1].push('V');
        }
        else{
            output[1].push('C');
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
        else if is_valid_cell(&target_cell, n_rows, n_cols) && is_valid_cell(&cell1, n_rows, n_cols){
            output.push(String::from("ok"));
        }
        else{
            output.push(String::from("Cell Out Of Bounds"));
        }
    }

    else if let Some(caps) = SCROLL_TO.captures(inp) {
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
    output
}


// pub fn main2(){
//     let s =String::from( "A3 = A4 + 5");
//     let outp = input(&s,10,10);
//     println!("{:?}",outp)
// }