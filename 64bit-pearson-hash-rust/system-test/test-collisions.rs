use std::io::process::Command;
use std::io::{File,BufferedReader};
use std::str::StrSplits;
use std::collections::HashSet;

fn main(){
    let countOutput = match Command::new("wc").arg("words.txt").output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute process: {}", e)
    };
    let string = String::from_utf8_lossy( countOutput.output.as_slice() ); // get output as string
    let mut split: StrSplits = string.split_str(" "); // split output from wc
    split.next(); // move iterator past leading whitespace
    let count: int = from_str( split.next().unwrap() ).unwrap(); // convert StrSplits iterator -> str -> int

    let mut reader = BufferedReader::new(File::open(&Path::new("words.txt")));
    let mut hashes: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        hashes.insert(line.unwrap());
    }

    println!("Number of words: {}", count);
    println!("Number of hashes: {}", hashes.len());
    println!("Number of collisions: {}", count - ( hashes.len() as int));

    let result = match Command::new("../target/pearson-64bit-hash").arg("asd").output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute process: {}", e)
    };
    println!("stdout: {}", String::from_utf8_lossy( result.output.as_slice() ));
}
