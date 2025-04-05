use regex::Regex;


fn is_integer(inp: &str) -> bool {
    let number = Regex::new(r"^-?\d+$").unwrap(); // regex for integer with optional unary minus
    number.is_match(inp)
}


fn input(inp: &str, n_rows: i32, n_cols: i32) -> Vec<&str> {

    // () used in regex for capture
    let output: Vec<&str>=Vec::new();
    let arithimetic = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)\s*([+\-*/])\s*(-?[A-Z]{1,3}[1-9][0-9]{0,2}|-?\d+)$").unwrap(); //regex for arth op
    let rangeop = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(MIN|MAX|AVG|SUM|STDEV)\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*:\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*\)$").unwrap(); //regex for range operation
    let assignment = Regex::new(r"^([A-Z]{1,3}[1-9][0-9]{0,2})\s*=\s*(-?\d+|[A-Z]{1,3}[1-9][0-9]{0,2})$").unwrap(); //regex for assignment
    let sleep = Regex::new(r"^SLEEP\s*\(\s*(?:[A-Z]{1,3}[1-9][0-9]{0,2}|\d+)\s*\)$").unwrap(); //regex for sleep
    let scroll_to = Regex::new(r"^scroll_to\s*\(\s*([A-Z]{1,3}[1-9][0-9]{0,2})\s*\)$").unwrap(); //regex for scroll_to

    // if let caps=arithimetic.captures(inp) {
    //     let mut target_cell = caps[1].to_string();
    //     let mut cell1  = caps[2].to_string();
    //     let mut cell2  = caps[4].to_string();
    //     let mut op = caps[3].to_string();


    // }


    return output;
}