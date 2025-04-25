//! This module contains functions to display a grid of data with labels.
//! It includes functions to shift characters for labeling columns and to display the grid with error handling.

/// Shifts a character by a given integer value.
/// # Arguments
/// * `c` - The character to be shifted.
/// * `i` - The integer value to shift the character by.
/// # Returns
/// The shifted character.
fn shift_char(c: char, i: i32) -> char {
    (c as i8 + i as i8) as u8 as char
}

/// Gets the label for a given integer.
/// # Arguments
/// * `a` - The integer to be converted to a label.
/// # Returns
/// A string representing the label.
/// The label is generated based on the integer value, with a specific mapping to letters.
/// The mapping is as follows:
/// - 1 to 26 maps to A to Z
/// - 27 to 702 maps to AA to ZZ
/// - 703 to 18277 maps to AAA to ZZZ
/// The function handles the conversion by calculating the appropriate letters based on the integer value.
/// The function uses a helper function `shift_char` to perform the character shifting.
/// The function returns a string representing the label.
pub fn get_label(a: i32) -> String {
    let mut temp = String::new();
    let mut num = a - 1;
    if (0..=25).contains(&num) {
        temp.push(shift_char('A', num));
    } else if (26..=701).contains(&num) {
        num -= 26;
        temp.push(shift_char('A', num / 26));
        temp.push(shift_char('A', num % 26));
    } else if (702..=18277).contains(&num) {
        num -= 702;
        let c = shift_char('A', num % 26);
        num /= 26;
        temp.push(shift_char('A', num / 26));
        temp.push(shift_char('A', num % 26));
        temp.push(c);
    }

    temp
}

/// Displays a grid of data with labels.
/// # Arguments
/// * `top_h` - The starting horizontal index.
/// * `top_v` - The starting vertical index.
/// * `len_h` - The length of the horizontal axis.
/// * `len_v` - The length of the vertical axis.
/// * `database` - A slice of integers representing the data.
/// * `err` - A slice of booleans representing error states for each data point.
/// # Returns
/// This function does not return a value.
/// It prints the grid to the console.
/// The grid is displayed with labels for the columns and rows.
/// The labels are generated using the `get_label` function.
/// The data points are displayed in the grid, with "ERR" printed for any data point that has an error.
pub fn display_grid(
    top_h: i32,
    top_v: i32,
    len_h: i32,
    len_v: i32,
    database: &[i32],
    err: &[bool],
) {
    let i1 = top_h;
    let mut i2 = top_h + 9;

    if i2 > len_h {
        i2 = len_h;
    }

    for i in i1..=i2 {
        print!("\t{}", get_label(i));
    }

    println!();

    let i3 = top_v;
    let mut i4 = top_v + 9;

    if i4 > len_v {
        i4 = len_v;
    }

    for j in i3..=i4 {
        print!("{j}");
        for i in i1..=i2 {
            if err[((j - 1) * len_h + i) as usize] {
                print!("\tERR");
            } else {
                print!("\t{}", database[((j - 1) * len_h + i) as usize]);
            }
        }
        println!();
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_char() {
        // Basic shift cases
        assert_eq!(shift_char('A', 0), 'A');
        assert_eq!(shift_char('A', 1), 'B');
        assert_eq!(shift_char('A', 25), 'Z');
        
        // Edge cases
        assert_eq!(shift_char('Z', 1), '['); // ASCII value after 'Z'
        assert_eq!(shift_char('A', -1), '@'); // ASCII value before 'A'
    }

    #[test]
    fn test_get_label_single_letter() {
        // Single letter cases (1-26)
        assert_eq!(get_label(1), "A");
        assert_eq!(get_label(2), "B");
        assert_eq!(get_label(26), "Z");
    }

    #[test]
    fn test_get_label_double_letter() {
        // Double letter cases (27-702)
        assert_eq!(get_label(27), "AA");
        assert_eq!(get_label(28), "AB");
        assert_eq!(get_label(52), "AZ");
        assert_eq!(get_label(53), "BA");
        assert_eq!(get_label(702), "ZZ");
    }


    #[test]
    fn test_get_label_triple_letter() {
        // Triple letter cases (703-18277)
        assert_eq!(get_label(703), "AAA");
        assert_eq!(get_label(704), "AAB");
        assert_eq!(get_label(728), "AAZ");
        assert_eq!(get_label(729), "ABA");
        assert_eq!(get_label(18278), "ZZZ");
    }

    #[test]
    fn test_get_label_boundary_cases() {
        // Test boundary cases between different label lengths
        assert_eq!(get_label(26), "Z");
        assert_eq!(get_label(27), "AA");
        assert_eq!(get_label(702), "ZZ");
        assert_eq!(get_label(703), "AAA");
        
    }

    
        
    #[test]
    fn test_display_grid() {
        
        // Create a small test dataset
        let len_h = 3;
        let len_v = 3;
        let database = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut err = vec![false; 10];
        err[4] = true; // Mark element at position (2,2) as error
        
        display_grid(1, 1, len_h, len_v, &database, &err);
        assert!(true); // If no panic occurs 

    }
    




}