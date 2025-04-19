fn shift_char(c:char, i: i32)-> char{
    (c as i8 + i as i8) as u8 as char
}

pub fn get_label(a : i32)-> String{
    let mut temp = String::new();
    let mut num = a-1;
    if (0..=25).contains(&num) {
        temp.push(shift_char('A', num));
    } else if (26..=701).contains(&num) {
        num-=26;
        temp.push(shift_char('A', num/26));
        temp.push(shift_char('A', num%26));
    } else if (702..=18277).contains(&num) {
        num-=702;
        let c = shift_char('A', num%26);
        num/=26;
        temp.push(shift_char('A', num/26));
        temp.push(shift_char('A', num%26));
        temp.push(c);
    }

    temp
}

pub fn display_grid(top_h: i32,top_v: i32,len_h: i32, len_v:i32, database: &Vec<i32>, err: &Vec<bool>){
    let i1 = top_h;
    let mut i2 = top_h + 9;

    if i2>len_h{i2=len_h;}

    for i in i1..=i2 {
        print!("\t{}",get_label(i));
    }

    println!();

    let i3 = top_v;
    let mut i4 = top_v + 9;

    if i4>len_v {i4=len_v;}

    for j in i3..=i4 {
        print!("{j}");
        for i in i1..=i2{
            if err[((j-1)*len_h + i) as usize] {
                print!("\tERR");
            }else{
            print!("\t{}",database[((j-1)*len_h + i) as usize]);
        }}
        println!();
    }
}
