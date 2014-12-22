use std::string::String;
use std::os;
//use std::rand::{task_rng, Rng};

//static mut T: [uint,..256] = [0,..256u];

static T: [uint,..256] = [58u, 153, 95, 18, 39, 195, 96, 20, 204, 161, 82, 19, 230, 238, 174, 81,
205, 0, 99, 28, 9, 103, 223, 27, 93, 229, 63, 35, 34, 178, 200, 138,
127, 225, 102, 77, 250, 206, 214, 17, 24, 124, 89, 193, 56, 110, 104, 61,
57, 137, 87, 88, 123, 232, 221, 255, 25, 51, 73, 226, 67, 71, 247, 112,
37, 31, 176, 182, 157, 29, 22, 197, 42, 111, 33, 186, 228, 148, 181, 212,
169, 198, 114, 94, 188, 231, 168, 8, 119, 216, 244, 249, 79, 55, 185, 7,
209, 211, 122, 202, 40, 133, 106, 11, 199, 235, 141, 241, 3, 91, 47, 49,
210, 78, 26, 43, 166, 2, 116, 144, 177, 5, 135, 184, 236, 208, 224, 219,
52, 237, 183, 72, 48, 53, 70, 125, 149, 30, 23, 151, 147, 220, 146, 12,
207, 159, 131, 69, 254, 74, 245, 191, 196, 139, 32, 60, 128, 172, 155, 16,
45, 44, 21, 38, 13, 97, 234, 173, 134, 15, 145, 248, 180, 105, 68, 92,
156, 150, 85, 251, 84, 233, 243, 46, 242, 162, 201, 152, 179, 4, 118, 164,
189, 109, 41, 80, 62, 222, 98, 6, 90, 101, 76, 83, 14, 160, 117, 59,
65, 246, 107, 1, 115, 194, 187, 253, 158, 142, 154, 66, 167, 10, 213, 217,
240, 227, 132, 252, 175, 130, 136, 113, 165, 50, 36, 163, 129, 121, 192, 126,
54, 218, 171, 100, 143, 75, 170, 190, 140, 64, 120, 215, 239, 108, 203, 86];

fn print_table(){
    unsafe {
        // Hardcoded width of array, as sqrt is not yet implemented for int trait.
        // Alternative would be to go via float;
        // let width = ((T.len() as f64).sqrt() as uint);
        for i in range(0, 16) {
            for j in range(0,T.len()/16){
                print!("{}, ", T[i*16 + j]);
            }
            println!("")
        }
    }
}


#[doc = "
Run only once, to create the randomized [0..255] table
"
]
/*
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
*/

fn generate_hash(string: String) -> String{
    let bytes = string.as_bytes();
    let mut hashed: [uint,..8] = [0u,..8];
    unsafe {
        for i in range(0u,8) {
            let mut h: uint = T[((bytes[0] as uint)+i) % 256];
            for j in range( 1, string.len() ) {
                h = T[h ^ (bytes[j] as uint)];
            }
            hashed[i] = h;
        }
    }
    format!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", hashed[0] as int,hashed[1] as int,hashed[2] as int,hashed[3] as int,hashed[4] as int,hashed[5] as int,hashed[6] as int,hashed[7] as int)
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Pass the string to be hashed as a command line argument");
        return;
    }
    let string: String = format!("{}", args[1]);
    println!("Input string: \t{}", string);

    let hashed: String = generate_hash(string);
    println!("Hashed: \t{}", hashed);
}
