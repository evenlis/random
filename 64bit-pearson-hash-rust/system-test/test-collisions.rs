use std::io::process::Command;
use std::io::{File,BufferedReader};
use std::str::StrSplits;
use std::collections::HashSet;
use std::os;

fn main(){

    // Stupid way to get line count when the file will be read anyway, derpderpderp. Oh well, at least
    // I got to learn how to run an arbitrary command
    /*
    let countOutput = match Command::new("wc").arg("words.txt").output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute process: {}", e)
    };
    let string = String::from_utf8_lossy( countOutput.output.as_slice() ); // get output as string
    let mut split: StrSplits = string.split_str(" "); // split output from wc
    split.next(); // move iterator past leading whitespace
    let count: int = from_str( split.next().unwrap() ).unwrap(); // convert StrSplits iterator -> str -> int
    */

    let args = os::args();
    let path:Path;

    if args.len()<2 {
        path = Path::new("words.txt");
    } else {
        path = Path::new(format!("{}", args[1]));
    }

    let mut reader = BufferedReader::new(File::open(&path));
    let mut hashes: HashSet<String> = HashSet::new();
    let mut lineCount = 0i;
    for line in reader.lines() {
        hashes.insert(line.unwrap());
        lineCount += 1;
    }

    println!("Number of words: {}", lineCount);
    println!("Number of hashes: {}", hashes.len());
    let collisions: int = lineCount - ( hashes.len() as int);
    println!("Number of collisions: {}", collisions);
    println!("Collision rate: {}%", (collisions/lineCount)*100);

    let result = match Command::new("../target/pearson-64bit-hash").arg("asd").output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute process: {}", e)
    };
    println!("stdout: {}", String::from_utf8_lossy( result.output.as_slice() ));
}
