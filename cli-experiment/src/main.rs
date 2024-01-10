use clap::{arg, command, Command, value_parser};

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            arg!([PORT])
                .value_parser(value_parser!(u16))
                .default_value("2020"),
        )
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    println!(
        "port: {:?}",
        matches
            .get_one::<u16>("PORT")
            .expect("default ensures there is always a value")
    );
}

