use std::{env, sync::Arc};

use crate::common::ConversionMapper;

mod common;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];

    let bytes = std::fs::read(file_name)
        .expect("Failed to read file!");

    let string = String::from_utf8(bytes).expect("Invalid file contents! Not UTF-8?");
    let mapper = Arc::new(ConversionMapper::from(&string[..]));

    let min_seed = mapper.seeds.iter().map(|v| mapper.lookup(*v)).min();
    println!("Lowest seed: {:?}", min_seed);
}
