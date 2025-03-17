use std::env;

mod types;
use types::{Clix};


pub fn entry() {
    let cli = Clix::new("clix", "0.1.0")
        .edit_about("A dragon in the making");
        
}

