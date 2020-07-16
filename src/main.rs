extern crate colored;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

/**
 * Options struct in which arguments are parsed
 */
#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Hey there!")]
    ///What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    ///Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    ///loads a custom cat image from specified file
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    ///Read message from STDIN instead of argument
    stdin: bool,
}

//Returns an ExitFailure
fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin()
            .read_to_string(&mut message)
            .with_context(|_| format!("could not receive input"))?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
    }

    let eye = if options.dead { "x" } else { "0" };

    match &options.catfile {
        Some(path) => {
            let template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read from file {:?}", path))?;

            let cat = template.replace("{eye}", eye);
            println!("{}", message.bright_blue().underline().italic().bold());
            println!("{}", &cat);
        }
        None => {
            println!("{}", message.bright_blue().underline().italic());
            println!("  \\");
            println!("   \\");
            println!("      /\\_/\\");
            println!("     ( {eye} {eye} )", eye = eye.red().bold());
            println!("     =( I )=");
        }
    }

    Ok(())
}
