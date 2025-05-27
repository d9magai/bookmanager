use derive_new::new;
use uuid::Uuid;

use crate::database::ConnectionPool;
use kernel::entity::books::Model;
use kernel::model::book::event::CreateBook;
use kernel::repository::book::BookRepository;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, book: CreateBook) -> Result<(), sqlx::Error> {
        // TODO: Implement the logic to create a book in the database
        unimplemented!()
    }

    async fn find_all(&self) -> Result<Vec<Model>, sqlx::Error> {
        // TODO: Implement the logic to retrieve all books from the database
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sqlx::Error> {
        // TODO: Implement the logic to find a book by its ID
        unimplemented!()
    }
}
