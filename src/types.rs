use clap;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
            .header(clap_cargo::style::HEADER)
            .usage(clap_cargo::style::USAGE)
            .literal(clap_cargo::style::LITERAL)
            .placeholder(clap_cargo::style::PLACEHOLDER)
            .error(clap_cargo::style::ERROR)
            .valid(clap_cargo::style::VALID)
            .invalid(clap_cargo::style::INVALID);


pub struct Clix {
    pub name: &str,

    pub version: String,

    pub subcommand_required: bool,
    
    pub command_required: bool,

}




pub struct Subcommand {
    pub name: &str,


    pub parameters: bool,
    pub options: bool,

}

impl Clix {
    pub fn new() {
        Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
    }
}