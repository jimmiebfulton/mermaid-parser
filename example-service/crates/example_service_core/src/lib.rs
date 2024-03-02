mod conversion;
mod r#impl;
pub mod settings;

use anyhow::Result;

use crate::settings::CoreSettings;
use example_service_persistence::ExampleServicePersistence;

pub mod proto {
    tonic::include_proto!("example.service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("example.service");
}

#[derive(Clone, Debug)]
pub struct ExampleServiceCore {
    persistence: ExampleServicePersistence,
}

impl ExampleServiceCore {
    pub fn builder(persistence: ExampleServicePersistence) -> Builder {
        Builder::new(persistence)
    }
}

pub struct Builder {
    persistence: ExampleServicePersistence,
    settings: CoreSettings,
}

impl Builder {
    pub fn new(persistence: ExampleServicePersistence) -> Self {
        Self {
            persistence,
            settings: Default::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<ExampleServiceCore> {
        Ok(ExampleServiceCore {
            persistence: self.persistence,
        })
    }
}