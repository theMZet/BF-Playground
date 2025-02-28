use std::{env::args, io::{stdin, stdout}};

use bf_playground::BFPlayground;


fn main() {
    let path = match args().skip(1).next() {
        Some(path) => {
            path
        },
        None => {
            println!("No path provided");
            return;
        }
    };

    let code = match std::fs::read_to_string(&path) {
        Ok(contents) => {
            contents
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let mut playground: BFPlayground = BFPlayground::new();
    playground.execute(&code, &mut stdin(), &mut stdout());
}