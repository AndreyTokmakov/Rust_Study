
use std::path::PathBuf;
use clap::{Arg, Command, builder::PathBufValueParser, ArgMatches};


fn parse_arguments()
{
    let matches: ArgMatches = Command::new("My Test Program")
        .version("1.2.3")
        .about("Teaches argument parsing")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("A cool file")
            .value_parser(PathBufValueParser::default()))
        .arg(Arg::new("num")
            .short('n')
            .long("number")
            .help("Five less than your favorite number"))
        .get_matches();

    let default_file: PathBuf = PathBuf::from("input.txt");
    let my_file: &PathBuf = matches.get_one("file").unwrap_or(&default_file);
    println!("The file passed is: {}", my_file.display());

    let num_str: Option<&String> = matches.get_one("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            let parsed: Result<i32, _> = s.parse();
            match parsed {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}

// INFO: https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
pub fn test_all()
{
    parse_arguments();
}