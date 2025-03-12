// use std::collections::HashMap;


pub struct Layer{
    pub order: i32,
    pub name: String,
    
    pub prefix: Option<String>,

    pub commands: Vec<String>,

    pub optional: bool
}

// PREFIX ORDER = -, +, --, ++, ---, +++

pub struct Clix{
    pub name: String,
    pub version: String,

    pub total: Vec<Layer>,

}

