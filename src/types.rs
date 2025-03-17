use core::result::Result;



use clap;
use clap_cargo;
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);


pub struct Clix<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub about: String,

    // pub subcommand_required: bool,

    // pub command_required: bool,
    pub subcmd_req: bool,

    pub subcommands: Vec<Subcommand<'a>>,

    command: Option<clap::Command>,
}

pub struct Subcommand<'a> {
    pub name: &'a str,
    pub about: &'a str,

    pub args: Option<Vec<String>>,

    pub subcommands: Option<Vec<Subcommand<'a>>>,
    // pub options: bool,
}

impl<'a> Clix<'a> {
    pub fn new(name: &'a str, version: &'a str) -> Self {
        Clix {
            name,
            version,
            about: String::from("A dragon in the making"),
            subcmd_req: false,
            // command_required: false,
            subcommands: Vec::new(),

            command: None,
        }
    }

    pub fn edit_about(mut self, about: &'a str) -> Self {
        self.about = about.to_owned();
        self
    }

    // pub fn build(mut self) -> Self {
    //     let mut app = clap::Command::new("git")
    //         .about("A fictional versioning CLI")
    //         .version(self.version)
    //         .subcommand_required(self.subcmd_req);

    //     for subcmd in self.subcommands.iter() {
    //         app.subcommand(clap::Command::new(subcmd.name).about(subcmd.about));
    //         // .args(subcmd.args)
    //         // .subcommands(subcmd.subcommands));
    //     }

    //     self.command = Some(app);
    //     self
    // }

    // pub fn get_subcommand(self, cmd: &str) -> Result<Self, &str> {
    //     let command = self.command;

    //     match command {
    //         Some(cmd) => {
    //             let matches = cmd.get_matches();
    //             match matches.subcommand() {
    //                 Some(subcmd) => return Ok(self),
    //                 None => return Err("Subcommand not found"),
    //             }
    //         }

    //         None => return Err("Command not found"),
    //     }

        // match matches.
    }
// }

impl<'a> Subcommand<'a> {
    pub fn new(name: &'a str, about: &'a str) -> Self {
        Subcommand {
            name,
            about,

            subcommands: None,
            args: None,
        }
    }
}
