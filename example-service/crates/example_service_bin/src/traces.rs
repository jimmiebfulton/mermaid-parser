#![allow(non_camel_case_types)]

use anyhow::Result;
use clap::{ArgEnum, PossibleValue};
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::settings::TraceSettings;

#[derive(Copy, Clone, Debug, ArgEnum, Serialize, Deserialize)]
pub enum TraceFormat {
    standard,
    json,
    pretty,
}

impl TraceFormat {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        TraceFormat::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl Default for TraceFormat {
    fn default() -> Self {
        TraceFormat::standard
    }
}

pub fn init(settings: &TraceSettings) -> Result<()> {
    let mut filter = EnvFilter::new(settings.filter());
    if let Ok(rust_log) = env::var(EnvFilter::DEFAULT_ENV) {
        filter = filter.add_directive(rust_log.parse()?);
    }

    match settings.format() {
        TraceFormat::standard => {
            tracing_subscriber::registry()
                .with(fmt::layer().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
        TraceFormat::json => {
            tracing_subscriber::registry()
                .with(fmt::layer().json().flatten_event(true))
                .with(filter)
                .init();
        }
        TraceFormat::pretty => {
            tracing_subscriber::registry()
                .with(fmt::layer().pretty().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
    };

    Ok(())
}