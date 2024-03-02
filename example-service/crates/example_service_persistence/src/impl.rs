use crate::sea_orm::entity::prelude::*;
use crate::Page;
use crate::{ExampleServicePersistence, DbResult};

use crate::entities::*;

impl ExampleServicePersistence {
    pub async fn find_example(
        &self,
        id: Uuid,
    ) -> DbResult<Option<example::Model>> {
        let record = example::Entity::find_by_id(id).one(self.connection()).await?;
        Ok(record)
    }

    pub async fn insert_example(
        &self,
        example_record: example::ActiveModel,
    ) -> DbResult<example::Model> {
        let result = example_record.insert(self.connection()).await?;
        Ok(result)
    }

    pub async fn update_example(
        &self,
        example_record: example::ActiveModel,
    ) -> DbResult<example::Model> {
        let result = example_record.update(self.connection()).await?;
        Ok(result)
    }

    pub async fn get_example_list(
        &self,
        page_size: usize,
        page: usize,
    ) -> DbResult<Page<example::Model>> {
        let paginator =
            example::Entity::find().paginate(self.connection(), if page_size > 0 { page_size } else { 10 });

        let records = paginator.fetch_page(page).await?;
        let total_pages = paginator.num_pages().await?;

        Ok(Page { records, total_pages })
    }
}