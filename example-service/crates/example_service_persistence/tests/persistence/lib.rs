use anyhow::Result;
use example_service_persistence::entities::*;
use example_service_persistence::sea_orm::prelude::*;
use example_service_persistence::sea_orm::*;
use example_service_persistence::{ExampleServicePersistence, DbResult, Page};

#[tokio::test]
async fn test_insert_example() -> Result<()> {
    let persistence = persistence().await?;

    let example = insert_example(&persistence).await?;
    assert_eq!(&example.contents, "Hello, World!");

    println!("{:?}", example);
    Ok(())
}

#[tokio::test]
async fn test_update_example() -> Result<()> {
    let persistence = persistence().await?;

    let example = insert_example(&persistence).await?;
    assert_eq!(&example.contents, "Hello, World!");

    let mut example = example.into_active_model();
    example.contents = Set("Goodbye, World!".to_owned());
    let example = persistence.update_example(example).await?;
    assert_eq!(&example.contents, "Goodbye, World!");

    println!("{:?}", example);
    Ok(())
}

#[tokio::test]
async fn test_list_examples() -> Result<()> {
    let persistence = persistence().await?;

    let Page { records, total_pages } = persistence.get_example_list(10, 0).await?;
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let _ = insert_example(&persistence).await?;
    let Page { records, total_pages } = persistence.get_example_list(10, 0).await?;
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    for _ in 1..=14 {
        let _ = insert_example(&persistence).await?;
    }
    let Page { records, total_pages } = persistence.get_example_list(10, 0).await?;
    assert_eq!(records.len(), 10);
    assert_eq!(total_pages, 2);

    let Page { records, total_pages } = persistence.get_example_list(10, 1).await?;
    assert_eq!(records.len(), 5);
    assert_eq!(total_pages, 2);

    Ok(())
}

async fn insert_example(persistence: &ExampleServicePersistence) -> DbResult<example::Model> {
    let example_record = example::ActiveModel {
        id: Set(Uuid::new_v4()),
        contents: Set("Hello, World!".to_owned()),
    };

    persistence.insert_example(example_record).await
}

async fn persistence() -> Result<ExampleServicePersistence> {
    ExampleServicePersistence::builder()
        .with_temp_db()
        .build()
        .await
}