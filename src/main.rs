use std::os::windows::io::RawSocket;

mod utils;

struct OPS {
    opcpde: String,
    cell1: i32,
    cell2: i32,
}

fn cell_to_int(a: &str) -> i32{
    let mut col = 0;
    let mut b = a.chars();

    while let Some(c) = b.next() {
        let diff = c as i32 - 'A' as i32 + 1;
        if 1<=diff && diff<=26 {
            col *= 26;
            col += diff;
        } else {
            break;
        }
    }

    let row: i32 = b.collect::<String>().parse().unwrap_or(0);

    col * 1000 + row
    }


fn int_to_ind(a: i32,len_h: i32)->i32{
    (a/1000) + (a%1000 -1)*len_h
}

fn cell_to_ind(a: &str, len_h: i32) -> i32{
    int_to_ind(cell_to_int(a),len_h)
}

fn calc(cell: i32, database: &mut Vec<i32>, opers: &Vec<OPS>, len_h: i32, err: &mut Vec<bool>) {
    match opers[cell as usize].opcpde.as_str() {
        "CCA" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1 as usize] + database[cell2 as usize];
        },
        "CVA" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] + opers[cell as usize].cell2;
        },
        "VCA" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = database[cell2] + opers[cell as usize].cell1;
        },
        "VVA" => {
            database[cell as usize] = opers[cell as usize].cell1 + opers[cell as usize].cell2;
        },
        "CCS" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] - database[cell2];
        },
        "CVS" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] - opers[cell as usize].cell2;
        },
        "VCS" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = opers[cell as usize].cell1 - database[cell2];
        },
        "VVS" => {
            database[cell as usize] = opers[cell as usize].cell1 - opers[cell as usize].cell2;
        },
        "CCM" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] * database[cell2];
        },
        "CVM" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1] * opers[cell as usize].cell2;
        },
        "VCM" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2];
            database[cell as usize] = opers[cell as usize].cell1 * database[cell2];
        },
        "VVM" => {
            database[cell as usize] = opers[cell as usize].cell1 * opers[cell as usize].cell2;
        },
        "CCD" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2] || database[cell2] == 0;
            if database[cell2] != 0 {
                database[cell as usize] = database[cell1] / database[cell2];
            }
        },
        "CVD" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1] || opers[cell as usize].cell2 == 0;
            if opers[cell as usize].cell2 != 0 {
                database[cell as usize] = database[cell1] / opers[cell as usize].cell2;
            }
        },
        "VCD" => {
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell2] || database[cell2] == 0;
            if database[cell2] != 0 {
                database[cell as usize] = opers[cell as usize].cell1 / database[cell2];
            }
        },
        "VVD" => {
            err[cell as usize] = opers[cell as usize].cell2 == 0;
            if opers[cell as usize].cell2 != 0 {
                database[cell as usize] = opers[cell as usize].cell1 / opers[cell as usize].cell2;
            }
        },
        "EQC" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            err[cell as usize] = err[cell1];
            database[cell as usize] = database[cell1];
        },
        "EQV" => {
            err[cell as usize] = false;
            database[cell as usize] = opers[cell as usize].cell1;
        },
        "MIN" => {
            database[cell as usize] = utils::operations::min(opers[cell as usize].cell1, opers[cell as usize].cell2, database, len_h, err, cell);
        },
        "MAX" => {
            database[cell as usize] = utils::operations::max(opers[cell as usize].cell1, opers[cell as usize].cell2, database, len_h, err, cell);
        },
        "MEA" => {
            database[cell as usize] = utils::operations::avg(opers[cell as usize].cell1, opers[cell as usize].cell2, database, len_h, err, cell);
        },
        "SUM" => {
            database[cell as usize] = utils::operations::sum(opers[cell as usize].cell1, opers[cell as usize].cell2, database, len_h, err, cell);
        },
        "STD" => {
            database[cell as usize] = utils::operations::stdev(opers[cell as usize].cell1, opers[cell as usize].cell2, database, len_h, err, cell);
        },
        _ => {}
    }
}

fn val_update(topo_arr : &Vec<i32>,database: &mut Vec<i32>,opers: &Vec<OPS>,len_h: i32,err: &mut Vec<bool>){
    for i in 1..=topo_arr[0]{
        calc(topo_arr[i as usize],database,opers,len_h,err)
    }
}

fn cell_update(inp_arr: &Vec<String>, database: &mut Vec<i32>, sensi: &mut Vec<Vec<i32>> , opers: &mut Vec<OPS>,len_h: i32,indegree : &mut Vec<i32>, err: &mut Vec<bool>){
    let target = cell_to_ind(&inp_arr[0],len_h);
    let target = target as usize;
    // Storing temporary value of opers in case a cycle is present
    let rev = OPS{
        opcpde: opers[target].opcpde.clone(),
        .. opers[target]
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
        let x1 = (rev.cell1 % len_h) as usize;
        let x2 = (rev.cell2 % len_h) as usize;
        let y1 = (rev.cell1 / len_h) as usize;
        let y2 = (rev.cell2 / len_h) as usize;

        for i in x1..=x2 {
            for j in y1..=y2 {
                sensi[i + (j - 1) * len_h as usize].retain(|&x| x != target as i32);
            }
        }
    }

    // Adding items to sensitivity list
    if inp_arr[1].starts_with('C') {
        if !sensi[opers[target].cell1 as usize].contains(&(target as i32)) {
            sensi[opers[target].cell1 as usize].push(target as i32);
        }
    }

    if inp_arr[1].chars().nth(1) == Some('C') {
        if !sensi[opers[target].cell2 as usize].contains(&(target as i32)) {
            sensi[opers[target].cell2 as usize].push(target as i32);
        }
    }

    if inp_arr[1] == "EQC" {
        if !sensi[opers[target].cell1 as usize].contains(&(target as i32)) {
            sensi[opers[target].cell1 as usize].push(target as i32);
        }
    }

    if inp_arr[1] == "SLC" {
        if !sensi[opers[target].cell1 as usize].contains(&(target as i32)) {
            sensi[opers[target].cell1 as usize].push(target as i32);
        }
    }

    if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
        let x1 = (opers[target].cell1 % len_h) as usize;
        let x2 = (opers[target].cell2 % len_h) as usize;
        let y1 = (opers[target].cell1 / len_h) as usize;
        let y2 = (opers[target].cell2 / len_h) as usize;

        for i in x1..=x2 {
            for j in y1..=y2 {
                if !sensi[i + (j - 1) * len_h as usize].contains(&(target as i32)) {
                    sensi[i + (j - 1) * len_h as usize].push(target as i32);
                }
            }
        }
    }

    let topo = utils::toposort::topo_sort(&sensi, target as i32, indegree);

    if topo[0] == -1 {
        // Removing items from sensitivity list

        // Handling arithmetic
        if inp_arr[1].starts_with('C') {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].remove(0);
                }
            }
        }

        if inp_arr[1].chars().nth(1) == Some('C') {
            if let Some(first) = sensi[opers[target].cell2 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell2 as usize].remove(0);
                }
            }
        }

        // Handling eq
        if inp_arr[1] == "EQC" {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].remove(0);
                }
            }
        }

        // Handling sleep
        if inp_arr[1] == "SLC" {
            if let Some(first) = sensi[opers[target].cell1 as usize].first() {
                if *first == target as i32 {
                    sensi[opers[target].cell1 as usize].remove(0);
                }
            }
        }

        // Handling ranges
        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
            let x1 = (opers[target].cell1 % len_h) as usize;
            let x2 = (opers[target].cell2 % len_h) as usize;
            let y1 = (opers[target].cell1 / len_h) as usize;
            let y2 = (opers[target].cell2 / len_h) as usize;

            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
                let xx1 = (rev.cell1 % len_h) as usize;
                let xx2 = (rev.cell2 % len_h) as usize;
                let xy1 = (rev.cell1 / len_h) as usize;
                let xy2 = (rev.cell2 / len_h) as usize;

                for i in x1..=x2 {
                    for j in y1..=y2 {
                        if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                            sensi[i + (j - 1) * len_h as usize].remove(0);
                        }
                    }
                }
            } else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        sensi[i + (j - 1) * len_h as usize].remove(0);
                    }
                }
            }
        }

        // Adding back older values

        if rev.opcpde.starts_with('C') {
            if !sensi[rev.cell1 as usize].contains(&(target as i32)) {
                sensi[rev.cell1 as usize].insert(0, target as i32);
            }
        }

        if rev.opcpde.chars().nth(1) == Some('C') {
            if !sensi[rev.cell2 as usize].contains(&(target as i32)) {
                sensi[rev.cell2 as usize].insert(0, target as i32);
            }
        }

        // Handling eq
        if rev.opcpde == "EQC" {
            if !sensi[rev.cell1 as usize].contains(&(target as i32)) {
                sensi[rev.cell1 as usize].insert(0, target as i32);
            }
        }

        // Handling sleep
        if rev.opcpde == "SLC" {
            if !sensi[rev.cell1 as usize].contains(&(target as i32)) {
                sensi[rev.cell1 as usize].insert(0, target as i32);
            }
        }

        // Handling ranges
        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
            let x1 = (rev.cell1 % len_h) as usize;
            let x2 = (rev.cell2 % len_h) as usize;
            let y1 = (rev.cell1 / len_h) as usize;
            let y2 = (rev.cell2 / len_h) as usize;

            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
                let xx1 = (opers[target].cell1 % len_h) as usize;
                let xx2 = (opers[target].cell2 % len_h) as usize;
                let xy1 = (opers[target].cell1 / len_h) as usize;
                let xy2 = (opers[target].cell2 / len_h) as usize;

                for i in x1..=x2 {
                    for j in y1..=y2 {
                        if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                            sensi[i + (j - 1) * len_h as usize].insert(0, target as i32);
                        }
                    }
                }
            } else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        sensi[i + (j - 1) * len_h as usize].insert(0, target as i32);
                    }
                }
            }
        }

        // Restoring back previous ops in case of cycle
        opers[target] = OPS{
            opcpde: rev.opcpde.clone(),
            ..rev
        }
    } else {
        val_update(&topo, database, opers, len_h, err);
    }




}

fn non_ui() {
    let len_h: i32 = 10;
    let len_v: i32 = 10;
    let mut database = vec![0; (len_h * len_v + 1) as usize];
    let mut err = vec![false; (len_h * len_v + 1) as usize];
    utils::display::display_grid(1, 1, len_h, len_v, &database, &err);
}

fn main() {
    non_ui();
}
