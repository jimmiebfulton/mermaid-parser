use anyhow::Result;
use example_service_core::proto::example_service_server::ExampleService;
use example_service_core::proto::{
    CreateExampleRequest, CreateExampleResponse, GetExampleListRequest,
    GetExampleListResponse,
};
use example_service_core::ExampleServiceCore;
use example_service_persistence::ExampleServicePersistence;
use tonic::Request;

#[tokio::test]
async fn test_create_example() -> Result<()> {
    let core = core().await?;

    let response = core
        .get_example_list(Request::new(GetExampleListRequest { page_size: 0, page: 0 }))
        .await?;
    let GetExampleListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let response = core
        .create_example(Request::new(CreateExampleRequest {
            contents: "Contents".to_string(),
        }))
        .await?;
    let CreateExampleResponse { record } = response.into_inner();
    let record = record.expect("Record Expected");
    assert_eq!(&record.contents, "Contents");

    let response = core
        .get_example_list(Request::new(GetExampleListRequest { page_size: 0, page: 0 }))
        .await?;
    let GetExampleListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    Ok(())
}

async fn core() -> Result<ExampleServiceCore> {
    let persistence = ExampleServicePersistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = ExampleServiceCore::builder(persistence)
        .build()
        .await?;
    Ok(core)
}