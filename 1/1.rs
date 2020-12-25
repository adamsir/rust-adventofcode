use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    let kokot = file.read_to_string(&mut contents)?;
     
    println!("{}", kokot);

    Ok(())
}
