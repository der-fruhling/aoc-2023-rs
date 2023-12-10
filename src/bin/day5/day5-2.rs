use std::{env, sync::Arc, io::{self, Write}, path::PathBuf};

use clap::Parser;

use crate::common::ConversionMapper;

mod common;

#[derive(clap::Parser)]
struct Cli {
    /// Disables printing debug messages while processing.
    ///
    /// May be useful for benchmarking the implementation.
    #[arg(long)]
    no_printing: bool,

    file_path: PathBuf
}

fn main() {
    let cli = Cli::parse();
    let bytes = std::fs::read(cli.file_path)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    let mapper = Arc::new(ConversionMapper::from(&string[..]));

    let seed_ranges = mapper.seeds.chunks_exact(2).map(|v| (v[0], v[1]));
    let mut smallest = i64::MAX;
    
    if !cli.no_printing {
        println!("Beginning magic");
    }
    
    let mut total_iterations = 0u128;
    
    for (start, count) in seed_ranges {
        if !cli.no_printing {
            println!("\x1b[0G\x1b[2KStarting range {}..{}", start, count);
            print!("Current smallest: {} (iteration: 0/{})", smallest, count);
        }
        
        for i in start..(start + count) {
            let result = mapper.lookup(i);
            
            if result < smallest {
                if !cli.no_printing {
                    println!("\x1b[0G\x1b[2K{} produced {} (less than {})", i, result, smallest);
                    print!("Current smallest: {}", smallest);
                }
                smallest = result;
            } else if total_iterations % (1 << 18) == 0 && !cli.no_printing {
                print!("\x1b[0G\x1b[2KCurrent smallest: {} (iteration: {}/{})", smallest, i - start, count);
                io::stdout().flush().expect("No stdout to flush!");
            } else if !cli.no_printing {
                total_iterations += 1;
            }
        }
    }
    
    if cli.no_printing {
        println!("{}", smallest);
    } else {
        println!("\x1b[0G\x1b[2KResult: {}", smallest);
    }
}
