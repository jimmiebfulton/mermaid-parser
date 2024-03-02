use tonic::{Request, Response, Status};

use example_service_persistence::{entities::*, sea_orm::prelude::*, sea_orm::*, Page};

use crate::{
    ExampleServiceCore,
    proto::{
        CreateExampleRequest, CreateExampleResponse,
        example_service_server::ExampleService, GetExampleListRequest, GetExampleListResponse,
    },
};
use crate::conversion::{ConvertFrom, ConvertTo};
use crate::proto::{Example, FindExampleRequest, FindExampleResponse, UpdateExampleRequest, UpdateExampleResponse};

#[tonic::async_trait]
impl ExampleService for ExampleServiceCore {
    async fn find_example(&self, request: Request<FindExampleRequest>) -> Result<Response<FindExampleResponse>, Status> {
        let request = request.into_inner();
        let id = request.id.convert_to()?;
        let result = self.persistence.find_example(id).await;
        match result {
            Ok(result) => {
                match result {
                    None => Err(Status::not_found("Record not found".to_owned())),
                    Some(model) => {
                        Ok(Response::new(FindExampleResponse {
                            record: Some(Example::convert_from(model))
                        }))
                    }
                }
            }
            Err(err) => {
                match err {
                    DbErr::RecordNotFound(err) => Err(Status::not_found(err)),
                    _ => Err(Status::internal("Unexpected error")),
                }
            }
        }
    }

    async fn create_example(
        &self,
        request: Request<CreateExampleRequest>,
    ) -> Result<Response<CreateExampleResponse>, Status> {
        let request = request.into_inner();
        tracing::info!("Received: {:?}", request);

        let example_record = example::ActiveModel {
            id: Set(Uuid::new_v4()),
            contents: Set(request.contents),
        };

        let result = self.persistence.insert_example(example_record).await;
        if let Ok(entity) = result {
            return Ok(Response::new(CreateExampleResponse {
                record: Some(Example::convert_from(entity)),
            }));
        }

        Err(Status::internal("Unexpected Error"))
    }

    async fn update_example(
        &self,
        request: Request<UpdateExampleRequest>,
    ) -> Result<Response<UpdateExampleResponse>, Status> {
        let example_record: example::ActiveModel =
            request.into_inner().record.unwrap().convert_to()?;
        let result = self
            .persistence
            .update_example(example_record.into_active_model())
            .await;

        match result {
            Ok(entity) => Ok(Response::new(UpdateExampleResponse {
                record: Some(Example::convert_from(entity)),
            })),
            Err(err) => match err {
                DbErr::RecordNotFound(err) => Err(Status::not_found(err)),
                _ => Err(Status::internal("Unexpected error")),
            },
        }
    }

    async fn get_example_list(
        &self,
        request: Request<GetExampleListRequest>,
    ) -> Result<Response<GetExampleListResponse>, Status> {
        let request = request.into_inner();
        tracing::info!("Received: {:?}", request);

        let response = self
            .persistence
            .get_example_list(request.page_size as usize, request.page as usize)
            .await;

        match response {
            Ok(Page { records, total_pages }) => {
                let records = records.into_iter().map(Example::convert_from).collect();
                Ok(Response::new(GetExampleListResponse {
                    records: records,
                    total_pages: total_pages as u32,
                }))
            }
            Err(_) => Err(Status::internal("Unknown Error")),
        }
    }
}