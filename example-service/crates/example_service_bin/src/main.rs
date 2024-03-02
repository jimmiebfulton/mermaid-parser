use anyhow::Result;

use example_service_core::ExampleServiceCore;
use example_service_persistence::ExampleServicePersistence;
use example_service_server::ExampleServiceServer;

mod cli;
mod settings;
mod traces;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = cli::arg_matches();
    let mut settings = settings::Settings::new(&args)?;
    traces::init(settings.tracing())?;

    match args.subcommand() {
        Some(("migrate", args)) => match args.subcommand() {
            Some(("up", _args)) => {
                settings.persistence_mut().set_migrate(Some(false));
                ExampleServicePersistence::builder()
                    .with_settings(settings.persistence())
                    .build()
                    .await?
                    .migrate_up(None)
                    .await?;
            }
            Some(("down", args)) => {
                let steps = if args.is_present("all") { None } else { Some(1) };
                settings.persistence_mut().set_migrate(Some(false));
                ExampleServicePersistence::builder()
                    .with_settings(settings.persistence())
                    .build()
                    .await?
                    .migrate_down(steps)
                    .await?;
            }
            _ => unreachable!(),
        },
        Some(("config", args)) => match args.subcommand() {
            Some(("defaults", _)) => settings::Settings::default().print()?,
            Some(("merged", _)) => settings.print()?,
            Some(("generate", _)) => settings.generate()?,
            _ => unreachable!(),
        },
        Some((_command, _args)) => {
            unreachable!()
        }
        None => {
            tracing::info!("Initializing...");
            let persistence = ExampleServicePersistence::builder()
                .with_settings(settings.persistence())
                .build()
                .await?;
            let core = ExampleServiceCore::builder(persistence)
                .with_settings(settings.core())
                .build()
                .await?;
            let server = ExampleServiceServer::builder(core)
                .with_settings(settings.server())
                .build()
                .await?;

            tokio::select! {
                result = server.serve() => {
                  return result;
                },
                _ = tokio::signal::ctrl_c() => {
                    return Ok(());
                },
            }
        }
    }

    Ok(())
}