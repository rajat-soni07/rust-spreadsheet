// use crate::cell_to_int;

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


fn is_arth(input:&str) -> bool {
    for c in input.chars(){
        if c=='('{
            return false;
        }
    }
    true
}

fn is_scroll(input:&str) ->bool{
    // if input is found true by is_arth and it does not contain =, then it is scroll_to
    for c in input.chars(){
        if c=='='{return false;}
    }
    true
}

fn is_integer(input:&str)->bool{
    //need to change
    let mut first=1;
    for c in input.chars(){
        if first==1{
            if c=='-' || c=='+'{continue;}
            first=0;
        }
        
        if !c.is_ascii_digit(){
            return false;
        }
    }
    true
}



fn is_valid_cell(cell:&str, len_h:i32,len_v:i32) ->bool{
    // input no of rows,no of cols
    let n=cell.len();
    if n<2{return false;}
    let mut first=1;
    let mut state=0;
    for i in cell.chars(){
        if first==1{
            first=0;
            if !i.is_ascii_uppercase(){
                return false;
            }
            continue;
        }

        if state==0{
            if !i.is_ascii_uppercase(){
                state=1;
            }
        }
        else if !i.is_ascii_digit(){
                return false;
            }
        
    }
    if state==0{return false;}
    let k=cell_to_int(cell);
    let r=k%1000;let c=k/1000;
    if r<=len_v && c<=len_h{
        return true;
    }
    false

}

fn is_valid_range(cell1:&str, cell2:&str) ->bool{
    let k1=cell_to_int(cell1); let r1=k1%1000; let c1=k1/1000;
    let k2=cell_to_int(cell2); let r2=k2%1000; let c2=k2/1000;
    
    !(r1>r2 || c1>c2)
}


fn check_err(input:&str , output: &[String] ,len_h:i32,len_v : i32) -> String{
    let mut message = String::from("ok");
    let vec1 = ["MEA","STD","SUM","MIN","MAX",];
    let vec2 = ["VVA","CVA","VCA","CCA","VVS","CVS","VCS","CCS","VVM","CVM","VCM","CCM","VVD","CVD","VCD","CCD"];
    if output[1].len()!=3{
        message = String::from("Invalid Operation");return message;
    }
    if output[1]=="SRL"{
        let mut temp=String::new();
        for i in input.chars(){
            if i==' '{
                break;
            }
            temp.push(i);
        }
        if temp!="scroll_to"{
            message = String::from("Invalid Operation");
        }
    }
    else{
        if !is_valid_cell(&output[0], len_h, len_v){
            message = String::from("Assigned Cell out of bounds");return message;
        }

        if output[1]=="SLC" || output[1]=="EQC"{
            if !is_valid_cell(&output[2], len_h, len_v){
                message = String::from("Invalid Cell");return message;
            }
        }
        else if output[1]=="SLV" || output[1]=="EQV" {return message;}

        else if vec1.contains(&(output[1].as_str())){
            if !is_valid_range(&output[2], &output[3]){
                message = String::from("Invalid Range");return message;
            }
            return message;
        }

        else if vec2.contains(&(output[1].as_str())){
            let f=output[1].chars().next().unwrap();
            let s=output[1].chars().nth(1).unwrap();
            if f=='C'{
                if !is_valid_cell(&output[2], len_h, len_v){
                    message = String::from("Invalid Cell");return message;
                }
                return message;

            }

            if s=='C'{
                if !is_valid_cell(&output[3], len_h, len_v){
                    message = String::from("Invalid Cell");return message;
                }
                return message;
            }
            
        }
        else{
            message = String::from("Invalid Operation");return message;
        }




    }
    message
}






pub fn help_input(input:&str) -> Vec<String>{
    let mut output= vec![String::new(); 4];
    let input_arr: Vec<char> = input.chars().collect();
    let n = input_arr.len();
    if is_scroll(input){
        let mut i=0;
        output[1]=String::from("SRL");
        while i<n && input_arr[i]!=' '{
            i+=1;
        }
        // put the cell in output[0]- target cell
        i+=1;
        while i<n{

            output[0].push(input_arr[i]);
            i+=1;
        }
        return output;
    }
    let mut i=0;

    while i<n && input_arr[i]!='='{
        output[0].push(input_arr[i]);
        i+=1;
    }

    if is_arth(input) {
        i+=1;
        while i<n && input_arr[i]==' '{i+=1;}
        output[2].push(input_arr[i]);
        i+=1;
        let mut oper;
        if i==n {
            output[1].push('E');
            output[1].push('Q');
            if is_integer(&output[2]){
                output[1].push('V');

            }
            else{
                output[1].push('C');
            }
            return output;
        }
        while i<n && (input_arr[i]!='*' && input_arr[i]!='/' && input_arr[i]!='+' && input_arr[i]!='-') {
            output[2].push(input_arr[i]);i+=1;
            if i==n {
                output[1].push('E');
                output[1].push('Q');
                if is_integer(&output[2]){
                    output[1].push('V');
    
                }
                else{
                    output[1].push('C');
                }
                return output;
            }
        }

        oper=input_arr[i];
        if oper=='+'{
            oper='A';
        }
        else if oper=='-'{
            oper='S';
        }
        else if oper=='*'{
            oper='M';
        }
        else if oper=='/'{
            oper='D';
        }
        i+=1;
        while input_arr[i]==' '{i+=1;}
        while i<n {output[3].push(input_arr[i]); i+=1;}

        if is_integer(&output[2]){
            output[1].push('V');
        }
        else{output[1].push('C');}

        if is_integer(&output[3]){
            output[1].push('V');
        }
        else{output[1].push('C');}

        output[1].push(oper);

    }
    else{
        i+=1;
        while i<n && input_arr[i]==' '{i+=1;}
        while i<n && input_arr[i]!='('{
            output[1].push(input_arr[i]);
            i+=1;
        }
        i+=1;
        if output[1]==*"SLEEP"{
            output[1]=String::from("SL");
            while i<n && input_arr[i]!=')'{
                output[2].push(input_arr[i]);
                i+=1;
        }}

        else{
            while i<n && input_arr[i]!=':'{
                output[2].push(input_arr[i]);
                i+=1;
            }
            i+=1;
            while i<n && input_arr[i]!=')'{
                output[3].push(input_arr[i]);
                i+=1;
            }
        }


    }

    if output[1]==*"STDEV"{
        output[1]=String::from("STD");
    }
    else if output[1]==*"AVG"{
        output[1]=String::from("MEA");
    }
    else if output[1]==*"SL"{
        if is_integer(&output[2]){
            output[1].push('V');
        }
        else{
            output[1].push('C');
        }
    }



    output
}


pub fn input(input:&str,len_h:i32,len_v : i32) -> Vec<String>{
    let mut output=help_input(input);
    
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