use std::env;

mod types;
use types::{Layer, Clix};

use cla

pub fn entry() -> String{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    return args[2].clone();
}

