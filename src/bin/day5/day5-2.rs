use std::{env, sync::Arc, io::{self, Write}};

use crate::common::ConversionMapper;

mod common;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    let mapper = Arc::new(ConversionMapper::from(&string[..]));

    let seed_ranges = mapper.seeds.chunks_exact(2).map(|v| (v[0], v[1]));
    let mut smallest = i64::MAX;
    
    println!("Beginning magic");
    
    let mut total_iterations = 0u128;
    
    for (start, count) in seed_ranges {
        println!("\x1b[0G\x1b[2KStarting range {}..{}", start, count);
        print!("Current smallest: {} (iteration: 0/{})", smallest, count);
        
        for i in start..(start + count) {
            let result = mapper.lookup(i);
            
            if result < smallest {
                println!("\x1b[0G\x1b[2K{} produced {} (less than {})", i, result, smallest);
                print!("Current smallest: {}", smallest);
                smallest = result;
            } else if total_iterations % (1 << 18) == 0 {
                print!("\x1b[0G\x1b[2KCurrent smallest: {} (iteration: {}/{})", smallest, i - start, count);
                io::stdout().flush().expect("No stdout to flush!");
            }
            
            total_iterations += 1;
        }
    }
    
    println!("\x1b[0G\x1b[2KResult: {}", smallest)
}
