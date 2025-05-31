use derive_new::new;
use uuid::Uuid;

use crate::database::ConnectionPool;
use kernel::entity::books::{self, Model};
use kernel::model::book::event::CreateBook;
use kernel::repository::book::BookRepository;
use sea_orm::ActiveModelTrait;
use sea_orm::{EntityTrait, QueryOrder};

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
        let books = books::Entity::find()
            .order_by_asc(books::Column::CreatedAt)
            .all(self.db.inner_ref())
            .await?;
        Ok(books)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        let book = books::Entity::find_by_id(id)
            .one(self.db.inner_ref())
            .await?;
        Ok(book)
    }
}
