use derive_new::new;
use uuid::Uuid;

use crate::database::ConnectionPool;
use kernel::entity::books::{self, Model};
use kernel::model::book::event::CreateBook;
use kernel::repository::book::BookRepository;
use sea_orm::ActiveModelTrait;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<(), sea_orm::DbErr> {
        let book = books::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            title: sea_orm::ActiveValue::Set(event.title),
            author: sea_orm::ActiveValue::Set(event.author),
            isbn: sea_orm::ActiveValue::Set(event.isbn),
            description: sea_orm::ActiveValue::Set(event.description),
            created_at: sea_orm::ActiveValue::NotSet,
            updated_at: sea_orm::ActiveValue::NotSet,
        };
        book.insert(self.db.inner_ref()).await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        // TODO: Implement the logic to retrieve all books from the database
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        // TODO: Implement the logic to find a book by its ID
        unimplemented!()
    }
}
