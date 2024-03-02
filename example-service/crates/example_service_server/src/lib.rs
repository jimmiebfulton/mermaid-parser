use crate::settings::ServerSettings;
use anyhow::Result;
use example_service_core::{
    proto::example_service_server::ExampleServiceServer as ExampleServiceProtoServer,
    ExampleServiceCore,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;

use tonic::transport::Server;

pub mod settings;

#[derive(Clone)]
pub struct ExampleServiceServer {
    core: ExampleServiceCore,
    service_port: u16,
    listener: Arc<Mutex<Option<TcpListener>>>,
}

pub struct Builder {
    settings: ServerSettings,
    core: ExampleServiceCore,
}

impl Builder {
    pub fn new(core: ExampleServiceCore) -> Builder {
        Builder {
            settings: ServerSettings::default(),
            core,
        }
    }

    pub fn with_settings(mut self, settings: &ServerSettings) -> Builder {
        self.settings = settings.clone();
        self
    }

    pub fn with_random_port(mut self) -> Builder {
        self.settings.service_mut().set_port(0);
        self
    }

    pub async fn build(self) -> Result<ExampleServiceServer> {
        let listener = TcpListener::bind((self.settings.host(), self.settings.service().port())).await?;
        let addr = listener.local_addr()?;

        Ok(ExampleServiceServer {
            core: self.core,
            service_port: addr.port(),
            listener: Arc::new(Mutex::new(Some(listener))),
        })
    }
}

impl ExampleServiceServer {
    pub fn builder(core: ExampleServiceCore) -> Builder {
        Builder::new(core)
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub async fn serve(&self) -> Result<()> {
        let listener = self.listener.lock().await.take().expect("Listener Expected");

        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<ExampleServiceProtoServer<ExampleServiceCore>>()
            .await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(example_service_core::proto::FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        let server = Server::builder()
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(ExampleServiceProtoServer::new(self.core.clone()));

        tracing::info!("ExampleService started on {}", listener.local_addr()?);

        server.serve_with_incoming(TcpListenerStream::new(listener)).await?;

        Ok(())
    }
}