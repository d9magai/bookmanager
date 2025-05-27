use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::books;
use crate::model::book::event::CreateBook;

#[async_trait]
pub trait BookRepository: Send + Sync {
    /// 本を保存します。
    async fn create(&self, event: CreateBook) -> Result<(), sqlx::Error>;

    /// すべての本を返します。
    async fn find_all(&self) -> Result<Vec<books::Model>, sqlx::Error>;

    /// 指定したIDの本を返します。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<books::Model>, sqlx::Error>;
}
