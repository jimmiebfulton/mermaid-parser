use clap::{command, Arg, ArgMatches, Command};

use crate::traces::TraceFormat;

pub fn arg_matches() -> ArgMatches {
    command!()
        .name("example-service")
        .subcommand(
            Command::new("migrate")
                .subcommand_required(true)
                .about("Database Migrations")
                .subcommand(Command::new("up").about("Apply migrations"))
                .subcommand(
                    Command::new("down")
                        .about("Roll back migrations.  Rolls back a single migration at a time, by default.")
                        .arg(
                            Arg::new("all")
                                .help("Rollback ALL migrations.  This will effectively destroy your entire database!")
                                .long("all"),
                        ),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Configuration Operations")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("defaults").about("Displays the default settings"))
                .subcommand(Command::new("merged").about("Displays the effective settings from all merged sources."))
                .subcommand(
                    Command::new("generate")
                        .about("Generate the effective settings in an adjacent yml file, overwriting existing config."),
                ),
        )
        .arg(
            Arg::new("config-file")
                .help("Specifies additional configuration to merge.")
                .long("config-file")
                .short('c')
                .takes_value(true),
        )
        .arg(
            Arg::new("host")
                .help("The host the server listens on.")
                .long("host")
                .takes_value(true),
        )
        .arg(
            Arg::new("log-sql")
                .help("Turns sql logging on or off.")
                .long("log-sql")
                .default_missing_value("true")
                .possible_values(&["true", "false"]),
        )
        .arg(
            Arg::new("service-port")
                .help("Service Port")
                .short('p')
                .long("service-port")
                .takes_value(true)
                .validator(is_valid_port),
        )
        .arg(
            Arg::new("temp-db")
                .help("Initialize and migrate an ephemeral database")
                .long("temp-db")
                .default_missing_value("true")
                .possible_values(&["true", "false"]),
        )
        .arg(
            Arg::new("migrate")
                .help("Whether or not to automatically migrate the database")
                .long("migrate"),
        )
        .arg(
            Arg::new("database-url")
                .help("Database URL")
                .long("database-url")
                .takes_value(true),
        )
        .arg(
            Arg::new("tracing-format")
                .help("Specify logging format")
                .long("tracing-format")
                .possible_values(TraceFormat::possible_values())
                .ignore_case(false),
        )
        .arg(
            Arg::new("tracing-filter")
                .help("Specify logging and tracing level filters")
                .long("tracing-filter")
                .takes_value(true)
                .ignore_case(false),
        )
        .get_matches()
}

fn is_valid_port(value: &str) -> Result<(), String> {
    value
        .parse::<u16>()
        .map_err(|_| format!("Ports must be an integer between 0 and {}", u16::MAX))
        .map(|_| ())
}