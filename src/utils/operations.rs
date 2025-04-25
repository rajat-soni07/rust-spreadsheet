//! This module contains functions for performing various operations on a 2D data array.
//! The operations include finding the minimum, maximum, sum, average, and standard deviation of elements
//! within a specified range of the data array. The functions also handle error checking and return the results accordingly.

/// Find the minimum value in a specified range of the data array.
/// # Arguments
/// * `c1` - The starting cell index (1-based).
/// * `c2` - The ending cell index (1-based).
/// * `data_base` - A reference to the data array.
/// * `n_cols` - The number of cells in the data array.
/// * `err` - A mutable reference to a boolean array for error checking.
/// * `dest` - The destination index in the error array to store the error status.
/// # Returns
/// The minimum value found in the specified range.
/// If there is err in the range, it sets the error flag for the destination index and the return value is discarded by the caller.
pub fn min(c1: i32, c2: i32, data_base: &[i32], n_cols: i32, err: &mut [bool], dest: i32) -> i32 {
    let mut y1 = c1 / n_cols;
    let mut y2 = c2 / n_cols;
    let mut x1 = c1 % (n_cols);
    if x1 == 0 {
        x1 = n_cols;
    }
    let mut x2 = c2 % (n_cols);
    if x2 == 0 {
        x2 = n_cols;
    }
    if x1 != n_cols {
        y1 += 1;
    }
    if x2 != n_cols {
        y2 += 1;
    }

    let mut ans = i32::MAX;
    let mut yn = false;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            yn |= err[(i + (j - 1) * n_cols) as usize];
            if (data_base[(i + (j - 1) * n_cols) as usize]) < ans {
                ans = data_base[(i + (j - 1) * n_cols) as usize];
            }
        }
    }
    err[dest as usize] = yn;
    ans
}

/// Find the maximum value in a specified range of the data array.
/// # Arguments
/// * `c1` - The starting cell index (1-based).
/// * `c2` - The ending cell index (1-based).
/// * `data_base` - A reference to the data array.
/// * `n_cols` - The number of cells in the data array.
/// * `err` - A mutable reference to a boolean array for error checking.
/// * `dest` - The destination index in the error array to store the error status.
/// # Returns 
/// The maximum value found in the specified range of the data array.
/// If there is err in the range, it sets the error flag for the destination index and the return value is discarded by the caller.

pub fn max(c1: i32, c2: i32, data_base: &[i32], n_cols: i32, err: &mut [bool], dest: i32) -> i32 {
    let mut y1 = c1 / n_cols;
    let mut y2 = c2 / n_cols;
    let mut x1 = c1 % (n_cols);
    if x1 == 0 {
        x1 = n_cols;
    }
    let mut x2 = c2 % (n_cols);
    if x2 == 0 {
        x2 = n_cols;
    }
    if x1 != n_cols {
        y1 += 1;
    }
    if x2 != n_cols {
        y2 += 1;
    }

    let mut ans = i32::MIN;
    let mut yn = false;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            yn |= err[(i + (j - 1) * n_cols) as usize];
            if data_base[(i + (j - 1) * n_cols) as usize] > ans {
                ans = data_base[(i + (j - 1) * n_cols) as usize];
            }
        }
    }
    err[dest as usize] = yn;
    ans
}


/// Find the sum of all values in a specified range of the data array.
/// # Arguments
/// * `c1` - The starting cell index (1-based).
/// * `c2` - The ending cell index (1-based).
/// * `data_base` - A reference to the data array.
/// * `n_cols` - The number of cells in the data array.
/// * `err` - A mutable reference to a boolean array for error checking.
/// * `dest` - The destination index in the error array to store the error status.
/// # Returns
/// The sum of all values found in the specified range.
/// If there is err in the range, it sets the error flag for the destination index and the return value is discarded by the caller.
pub fn sum(c1: i32, c2: i32, data_base: &[i32], n_cols: i32, err: &mut [bool], dest: i32) -> i32 {
    let mut y1 = c1 / n_cols;
    let mut y2 = c2 / n_cols;
    let mut x1 = c1 % (n_cols);
    if x1 == 0 {
        x1 = n_cols;
    }
    let mut x2 = c2 % (n_cols);
    if x2 == 0 {
        x2 = n_cols;
    }
    if x1 != n_cols {
        y1 += 1;
    }
    if x2 != n_cols {
        y2 += 1;
    }

    let mut ans = 0;
    let mut yn = false;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            yn |= err[(i + (j - 1) * n_cols) as usize];
            ans += data_base[(i + (j - 1) * n_cols) as usize];
        }
    }
    err[dest as usize] = yn;
    ans
}


/// Find the average of all values in a specified range of the data array.
/// # Arguments
/// * `c1` - The starting cell index (1-based).
/// * `c2` - The ending cell index (1-based).
/// * `data_base` - A reference to the data array.
/// * `n_cols` - The number of cells in the data array.
/// * `err` - A mutable reference to a boolean array for error checking.
/// * `dest` - The destination index in the error array to store the error status.
/// # Returns
/// The average of all values found in the specified range.
/// If there is err in the range, it sets the error flag for the destination index and the return value is discarded by the caller.
pub fn avg(c1: i32, c2: i32, data_base: &[i32], n_cols: i32, err: &mut [bool], dest: i32) -> i32 {
    let mut y1 = c1 / n_cols;
    let mut y2 = c2 / n_cols;
    let mut x1 = c1 % (n_cols);
    if x1 == 0 {
        x1 = n_cols;
    }
    let mut x2 = c2 % (n_cols);
    if x2 == 0 {
        x2 = n_cols;
    }
    if x1 != n_cols {
        y1 += 1;
    }
    if x2 != n_cols {
        y2 += 1;
    }

    let mut ans = 0;
    let mut ct = 0;
    let mut yn = false;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            ct += 1;
            yn |= err[(i + (j - 1) * n_cols) as usize];
            ans += data_base[(i + (j - 1) * n_cols) as usize];
        }
    }
    err[dest as usize] = yn;
    ans / ct
}

/// Find the standard deviation of all values in a specified range of the data array.
/// # Arguments
/// * `c1` - The starting cell index (1-based).
/// * `c2` - The ending cell index (1-based).
/// * `data_base` - A reference to the data array.
/// * `n_cols` - The number of cells in the data array.
/// * `err` - A mutable reference to a boolean array for error checking.
/// * `dest` - The destination index in the error array to store the error status.
/// # Returns
/// The standard deviation of all values found in the specified range.
/// If there is err in the range, it sets the error flag for the destination index and the return value is discarded by the caller.
pub fn stdev(c1: i32, c2: i32, data_base: &[i32], n_cols: i32, err: &mut [bool], dest: i32) -> i32 {
    let mut y1 = c1 / n_cols;
    let mut y2 = c2 / n_cols;
    let mut x1 = c1 % (n_cols);
    if x1 == 0 {
        x1 = n_cols;
    }
    let mut x2 = c2 % (n_cols);
    if x2 == 0 {
        x2 = n_cols;
    }
    if x1 != n_cols {
        y1 += 1;
    }
    if x2 != n_cols {
        y2 += 1;
    }

    let mut var = 0.0;
    let mut ct = 0;
    let mut ans = 0;
    let mut yn = false;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            ct += 1;
            yn |= err[(i + (j - 1) * n_cols) as usize];
            ans += data_base[(i + (j - 1) * n_cols) as usize];
        }
    }
    let mean = ans / ct;
    for i in x1..x2 + 1 {
        for j in y1..y2 + 1 {
            yn |= err[(i + (j - 1) * n_cols) as usize];
            var += (data_base[(i + (j - 1) * n_cols) as usize] - mean) as f64
                * (data_base[(i + (j - 1) * n_cols) as usize] - mean) as f64;
        }
    }
    var /= ct as f64;
    err[dest as usize] = yn;

    var.sqrt() as i32
}
