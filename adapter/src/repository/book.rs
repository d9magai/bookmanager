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
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
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

#[cfg(test)]
mod tests {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use sea_orm::DatabaseConnection;

    use super::*;

    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::up(&db, None).await.unwrap();
        db
    }

    fn create_book_event() -> CreateBook {
        CreateBook {
            title: "Rust Book".to_string(),
            author: "Ferris".to_string(),
            isbn: "1234567890".to_string(),
            description: "A book about Rust.".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_and_find_by_id_and_find_all() {
        let db = setup_db().await;
        let pool = ConnectionPool::new(db);
        let repo = BookRepositoryImpl::new(pool);

        // book作成
        let event = create_book_event();
        repo.create(event.clone()).await.unwrap();

        // 全件取得
        let books = repo.find_all().await.unwrap();
        assert_eq!(books.len(), 1);
        let book = &books[0];
        assert_eq!(book.title, event.title);
        assert_eq!(book.author, event.author);
        assert_eq!(book.isbn, event.isbn);
        assert_eq!(book.description, event.description);

        // 1件取得
        let found = repo.find_by_id(book.id).await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.title, event.title);
        assert_eq!(found.author, event.author);
        assert_eq!(found.isbn, event.isbn);
        assert_eq!(found.description, event.description);
    }
}
