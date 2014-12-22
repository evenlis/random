use std::rand::{task_rng, Rng};
use std::num::Float;
use std::string::String;
use std::vec::Vec;
use std::c_str::CString;

static mut T: [uint,..256] = [0,..256u];

fn print_table(){
    unsafe {
        // Hardcoded width of array, as sqrt is not yet implemented for int trait.
        // Alternative would be to go via float;
        // let width = ((T.len() as f64).sqrt() as uint);
        for i in range(0, 16) {
            for j in range(0,T.len()/16){
                print!("{}\t", T[i*16 + j]);
            }
            println!("")
        }
    }
}

fn create_table(){
    let mut rng = task_rng();
    unsafe {
        for i in range(0,256){
            T[i] = i;
        }
        for i in range(0u,256) {
            let num: uint = rng.gen_range(0, 255);
            T.swap(i, num);
        }
    }

}


fn main() {
    create_table();
    print_table();
    let string: String = String::from_str("asd");
    let hex: String = String::from_str("sdf");
}
