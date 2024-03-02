use std::process;

use anyhow::Result;
use clap::{ArgMatches, Command};

const PG_USERNAME: &str = "test";
const PG_DOCKER_NAME: &'static str = "postgres-xtask";
const SERVICE_NAME: &str = "example-service";

fn main() -> Result<()> {
    let args = clap::command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("postgres")
                .about("Dockerized PostgreSQL Management")
                .subcommand(Command::new("run").about("Create and Start a PostgreSQL Docker Container"))
                .subcommand(Command::new("kill").about("Kill PostgreSQL Docker Container"))
                .subcommand(Command::new("stop").about("Stop PostgreSQL Docker Container"))
                .subcommand(Command::new("start").about("Start an existing PostgreSQL Docker Container"))
                .subcommand(Command::new("rm").about("Remove an existing PostgreSQL Docker Container"))
                .subcommand(Command::new("createdb").about("Creates the database in an already-running posgres"))
                .subcommand(Command::new("dropdb").about("Drops the database in an already-running posgres")),
        )
        .subcommand(
            Command::new("docker")
                .about("Docker Operations")
                .subcommand(Command::new("build").about("Builds an application Docker image."))
                .subcommand(Command::new("rmi").about("Removes the application Docker image.")),
        )
        .get_matches();

    match args.subcommand() {
        Some(("postgres", args)) => handle_postgres_commands(args),
        Some(("docker", args)) => handle_docker_commands(args),
        Some((command, _)) => anyhow::bail!("Unexpected command: {command}"),
        None => anyhow::bail!("Expected subcommand"),
    }
}

fn handle_postgres_commands(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("run", _)) => postgres_init(),
        Some(("createdb", _)) => postgres_createdb(),
        Some(("dropdb", _)) => postgres_dropdb(),
        Some((command, _)) => postgres_docker_command(command),
        None => anyhow::bail!("Expected subcommand"),
    }
}

fn postgres_createdb() -> Result<()> {
    process::Command::new("createdb")
        .arg("-U")
        .arg(PG_USERNAME)
        .arg("-h")
        .arg("localhost")
        .arg(SERVICE_NAME)
        .spawn()?
        .wait()?;

    println!("CLI: psql -U test -h localhost {}", SERVICE_NAME);

    Ok(())
}

fn postgres_dropdb() -> Result<()> {
    process::Command::new("dropdb")
        .arg("-U")
        .arg(PG_USERNAME)
        .arg("-h")
        .arg("localhost")
        .arg(SERVICE_NAME)
        .spawn()?
        .wait()?;

    Ok(())
}

fn handle_docker_commands(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("build", _args)) => docker_build(),
        Some(("rmi", _args)) => docker_rmi(),
        _ => Ok(()),
    }
}

fn docker_build() -> Result<()> {
    process::Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(SERVICE_NAME)
        .arg(".")
        .spawn()?
        .wait()?;

    Ok(())
}

fn docker_rmi() -> Result<()> {
    process::Command::new("docker")
        .arg("rmi")
        .arg(SERVICE_NAME)
        .spawn()?
        .wait()?;

    Ok(())
}

fn postgres_init() -> Result<()> {
    process::Command::new("docker")
        .arg("run")
        .arg("-e")
        .arg(format!("POSTGRES_USER={PG_USERNAME}"))
        .arg("-e")
        .arg("POSTGRES_HOST_AUTH_METHOD=trust")
        .arg("-e")
        .arg(format!("POSTGRES_DB={SERVICE_NAME}"))
        .arg("-p")
        .arg("127.0.0.1:5432:5432")
        .arg("--name")
        .arg(PG_DOCKER_NAME)
        .arg("-d")
        .arg("postgres:latest")
        .arg("-N")
        .arg("1000")
        .spawn()?
        .wait()?;

    println!("CLI: psql -U test -h localhost {}", SERVICE_NAME);

    Ok(())
}

fn postgres_docker_command(command: &str) -> Result<()> {
    process::Command::new("docker")
        .arg(command)
        .arg(PG_DOCKER_NAME)
        .spawn()?
        .wait()?;

    Ok(())
}