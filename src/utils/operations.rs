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
