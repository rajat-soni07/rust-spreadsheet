use std::io;
use std::io::Write;

mod utils;

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

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
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

fn int_to_ind(a: i32,len_h: i32)->i32{
    (a/1000) + (a%1000 -1)*len_h
}

fn cell_to_ind(a: &str, len_h: i32) -> i32{
    int_to_ind(cell_to_int(a),len_h)
}

fn calc(cell: i32, database: &mut [i32], opers: &[Ops], len_h: i32, err: &mut [bool]) {
    match opers[cell as usize].opcpde.as_str() {
        "CCA" => {
            let cell1 = opers[cell as usize].cell1 as usize;
            let cell2 = opers[cell as usize].cell2 as usize;
            err[cell as usize] = err[cell1] || err[cell2];
            database[cell as usize] = database[cell1] + database[cell2];
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
        "SLV" =>{
            std::thread::sleep(std::time::Duration::from_secs(max(0,opers[cell as usize].cell1) as u64));
            database[cell as usize] = opers[cell as usize].cell1;
            err[cell as usize] = false;
        }
        "SLC" =>{
            if err[opers[cell as usize].cell1 as usize]{
                err[cell as usize] = true;
            }else{
                std::thread::sleep(std::time::Duration::from_secs(max(0,database[opers[cell as usize].cell1 as usize]) as u64));
                database[cell as usize] = database[opers[cell as usize].cell1 as usize];
                err[cell as usize] = false;
            }
        }
        _ => {}
    }
}

fn val_update(topo_arr : &[i32],database: &mut [i32],opers: &[Ops],len_h: i32,err: &mut [bool]){
    for i in 1..=topo_arr[0]{
        calc(topo_arr[i as usize],database,opers,len_h,err)
    }
}

fn cell_update(inp_arr: &[String], database: &mut [i32], sensi: &mut [Vec<i32>] , opers: &mut [Ops],len_h: i32,indegree : &mut [i32], err: &mut [bool])->i32{
    let target = cell_to_ind(&inp_arr[0],len_h);
    let target = target as usize;
    // Storing temporary value of opers in case a cycle is present
    let rev = Ops{
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
        let mut x1 = (rev.cell1 % len_h) as usize;
        let mut x2 = (rev.cell2 % len_h) as usize;
        if x1==0{x1 = len_h as usize;}
        if x2==0{x2 = len_h as usize;}

        let y1 = (rev.cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
        let y2 = (rev.cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
            let mut xx1 = (opers[target].cell1 % len_h) as usize;
            let mut xx2 = (opers[target].cell2 % len_h) as usize;
            if xx1==0{xx1 = len_h as usize;}
            if xx2==0{xx2 = len_h as usize;}
    
            let xy1 = (opers[target].cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
            let xy2 = (opers[target].cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);
    
    
            for i in x1..=x2 {
                for j in y1..=y2 {
                    if !(xx1 <= i && i <= xx2 && xy1 <= j && j <= xy2) {
                        sensi[i + (j - 1) * len_h as usize].retain(|&x| x != target as i32);
                    }
                }
            }
        }else{
            for i in x1..=x2 {
                for j in y1..=y2 {
                    sensi[i + (j - 1) * len_h as usize].retain(|&x| x != target as i32);
                }
            }
        }
       
    }

    // Adding items to sensitivity list

    // Handling arithmetic
    if inp_arr[1].starts_with('C') && (sensi[opers[target].cell1 as usize].is_empty() || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32) {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    if inp_arr[1].chars().nth(1) == Some('C') && (sensi[opers[target].cell2 as usize].is_empty() || *sensi[opers[target].cell2 as usize].last().unwrap() != target as i32) {
        sensi[opers[target].cell2 as usize].push(target as i32);
    }

    // Handling eq
    if inp_arr[1] == "EQC" && (sensi[opers[target].cell1 as usize].is_empty() || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32) {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    if inp_arr[1] == "SLC" && (sensi[opers[target].cell1 as usize].is_empty() || *sensi[opers[target].cell1 as usize].last().unwrap() != target as i32) {
        sensi[opers[target].cell1 as usize].push(target as i32);
    }

    // Handling ranges
    if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
        let mut x1 = (opers[target].cell1 % len_h) as usize;
        let mut x2 = (opers[target].cell2 % len_h) as usize;
        if x1==0{x1 = len_h as usize;}
        if x2==0{x2 = len_h as usize;}

        let y1 = (opers[target].cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
        let y2 = (opers[target].cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
            let mut xx1 = (rev.cell1 % len_h) as usize;
            let mut xx2 = (rev.cell2 % len_h) as usize;
            if xx1==0{xx1 = len_h as usize;}
            if xx2==0{xx2 = len_h as usize;}
    
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
            if x1==0{x1 = len_h as usize;}
            if x2==0{x2 = len_h as usize;}

            let y1 = (opers[target].cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
            let y2 = (opers[target].cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);

            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
                let mut xx1 = (rev.cell1 % len_h) as usize;
                let mut xx2 = (rev.cell2 % len_h) as usize;
                if xx1==0{xx1 = len_h as usize;}
                if xx2==0{xx2 = len_h as usize;}
        
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

        if rev.opcpde.starts_with('C') && (sensi[rev.cell1 as usize].is_empty() || *sensi[rev.cell1 as usize].last().unwrap() != target as i32) {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        if rev.opcpde.chars().nth(1) == Some('C') && (sensi[rev.cell2 as usize].is_empty() || *sensi[rev.cell2 as usize].last().unwrap() != target as i32) {
            sensi[rev.cell2 as usize].push(target as i32);
        }

        // Handling eq
        if rev.opcpde == "EQC" && (sensi[rev.cell1 as usize].is_empty() || *sensi[rev.cell1 as usize].last().unwrap() != target as i32) {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        // Handling sleep
        if rev.opcpde == "SLC" && (sensi[rev.cell1 as usize].is_empty() || *sensi[rev.cell1 as usize].last().unwrap() != target as i32) {
            sensi[rev.cell1 as usize].push(target as i32);
        }

        // Handling ranges
        if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&rev.opcpde.as_str()) {
            let mut x1 = (rev.cell1 % len_h) as usize;
            let mut x2 = (rev.cell2 % len_h) as usize;
            if x1==0{x1 = len_h as usize;}
            if x2==0{x2 = len_h as usize;}

            let y1 = (rev.cell1 / len_h) as usize + ((x1 != len_h as usize) as usize);
            let y2 = (rev.cell2 / len_h) as usize + ((x2 != len_h as usize) as usize);


            if ["SUM", "MIN", "MAX", "MEA", "STD"].contains(&inp_arr[1].as_str()) {
                let mut xx1 = (opers[target].cell1 % len_h) as usize;
                let mut xx2 = (opers[target].cell2 % len_h) as usize;
                if xx1==0{xx1 = len_h as usize;}
                if xx2==0{xx2 = len_h as usize;}
        
                let xy1 = (opers[target].cell1 / len_h) as usize + ((xx1 != len_h as usize) as usize);
                let xy2 = (opers[target].cell2 / len_h) as usize + ((xx2 != len_h as usize) as usize);
    
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
        opers[target] = Ops{
            opcpde: rev.opcpde.clone(),
            ..rev
        };

        0
    } else {
        val_update(&topo, database, opers, len_h, err);
        1
    }




}

fn non_ui(len_h: i32, len_v: i32) {

    let mut database = vec![0; (len_h * len_v + 1) as usize];
    let mut err = vec![false; (len_h * len_v + 1) as usize];
    let mut opers = vec![Ops{opcpde: String::new(),cell1: -1, cell2 :-1}; (len_h * len_v + 1) as usize];
    let mut indegree = vec![0; (len_h * len_v + 1) as usize];
    let mut sensi = vec![Vec::<i32>::new();(len_h * len_v + 1) as usize];

    let mut curr_h = 1;
    let mut curr_v = 1;
    let mut status = String::from("ok");
    let mut dis = false;

    utils::display::display_grid(curr_h, curr_v, len_h, len_v, &database, &err);

    let mut time = 0.0;
    loop{
        print!("[{:.1}] ({}) > ", time, status);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim_end().to_string();
        let start_time = std::time::Instant::now();
        match input.as_str() {

            "w" => {
                curr_v = max(1,curr_v-10);
            },
            "a" => {
                curr_h = max(1,curr_h-10);
            },
            "s" => {
                if curr_v+10>len_v{curr_v = len_v - 9}else{curr_v += 10}
            },
            "d" => {
                if curr_h+10>len_h{curr_h = len_h - 9}else{curr_h += 10}
            },
            "q" => {
                break;
            },
            "disable_output" => {
                dis = true;
                status = "ok".to_string();
            },
            "enable_output" => {
                status = "ok".to_string();
                dis = false;
            },
            _ => {

                let out = utils::input::input(&input, len_h, len_v);
                status = out[4].clone();
                if status == "ok" {
                    if out[1] == "SRL"{
                        let t = cell_to_ind(out[0].as_str(), len_h);
                        let mut x1 = t%len_h; if x1==0{x1=len_h;}
                        let y1 = t/len_h + ((x1!=len_h) as i32);
                        curr_h = x1; curr_v = y1;
                        // println!("Scrolling to cell {} at ({},{})", out[0], curr_h, curr_v);
                    }
                    else{
                        let suc = cell_update(&out, &mut database, &mut sensi, &mut opers, len_h, &mut indegree, &mut err);
                        if suc==0{
                            status = "cycle_detected".to_string();
                        }
                    }
                }
            }
        }
        let end_time = std::time::Instant::now();
        time = (end_time - start_time).as_secs_f64();

        if dis{
            continue;
        }else{
            utils::display::display_grid(curr_h, curr_v, len_h, len_v, &database, &err);
        }
    }

}


fn ui(len_h: i32,len_v: i32) -> eframe::Result {
    let database = vec![0; (len_h * len_v + 1) as usize];
    let err = vec![false; (len_h * len_v + 1) as usize];
    let opers = vec![Ops{opcpde: String::new(),cell1: -1, cell2 :-1}; (len_h * len_v + 1) as usize];
    let indegree = vec![0; (len_h * len_v + 1) as usize];
    let sensi = vec![Vec::<i32>::new();(len_h * len_v + 1) as usize];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]).with_resizable(false).with_maximize_button(false),
        
        ..Default::default()
    };
    eframe::run_native(
        "Spreadsheet",
        options,
        
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(utils::ui::gui::Spreadsheet::new(len_h, len_v, database, err, opers, indegree, sensi)))
        }),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >=3{
        let len_h: i32 = args[2].parse().unwrap_or(10);
        let len_v: i32 = args[1].parse().unwrap_or(10);
        if args.len() == 4 {
            if args[3] == "--ui"{
                ui(len_h,len_v).unwrap();
            }             
        } else{
            non_ui(len_h,len_v);
        }
    } else {
        println!("Usage: cargo run <len_h> <len_v> <flag>");
    }
}
