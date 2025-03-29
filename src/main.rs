mod utils;

fn main() {
    let len_h :i32 = 10;
    let len_v :i32 = 10;
    let database = vec![0;(len_h*len_v + 1)as usize];
    utils::display::display_grid(1,1,len_h,len_v,&database);
}
