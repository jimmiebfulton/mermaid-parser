use anyhow::Result;
use example_service_client::proto::example_service_client::ExampleServiceClient;
use example_service_client::proto::CreateExampleRequest;
use example_service_core::ExampleServiceCore;
use example_service_persistence::ExampleServicePersistence;
use example_service_server::ExampleServiceServer;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::test]
async fn test_core() -> Result<()> {
    let (mut client, _) = init().await?;

    let request = Request::new(CreateExampleRequest {
        contents: "Contents".to_string(),
    });

    let response = client.create_example(request).await?;
    let response = response.into_inner();
    assert_eq!(response.record.unwrap().contents, "Contents".to_owned());

    Ok(())
}

async fn init() -> Result<(ExampleServiceClient<Channel>, ExampleServiceServer)> {
    let persistence = ExampleServicePersistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = ExampleServiceCore::builder(persistence)
        .build()
        .await?;
    let server = ExampleServiceServer::builder(core)
        .with_random_port()
        .build()
        .await?;

    let server_clone = server.clone();

    tokio::spawn(async move {
        let _ = server_clone.serve().await;
    });

    let addr = format!("http://localhost:{}", server.service_port());
    let client = ExampleServiceClient::connect(addr).await?;

    Ok((client, server))
}