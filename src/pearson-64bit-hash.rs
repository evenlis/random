use std::rand::{task_rng, Rng};
use std::num::Float;
use std::string::String;

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

fn generate_hash(string: String) -> [uint,..8]{
    let bytes = string.as_bytes();
    let mut hashed_array: [uint,..8] = [0u,..8];
    unsafe {
        for i in range(0u,8) {
            let mut h: uint = T[((bytes[0] as uint)+i) % 256];
            for j in range( 1, string.len() ) {
                h = T[h ^ (bytes[j] as uint)];
            }
            hashed_array[i] = h;
        }
    }
    hashed_array
}

fn main() {
    create_table();
    let string: String = String::from_str("asdasdasdsadasd");
    let hashed: [uint,..8] = generate_hash(string);
    for i in hashed.iter() {
        print!("{:x}", *i as int);
    }
}
