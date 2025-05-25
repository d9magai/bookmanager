use async_trait::async_trait;
use uuid::Uuid;

use crate::model::book::event::CreateBook;

#[async_trait]
pub trait BookRepository: Send + Sync {
    /// 本を保存します。
    async fn create(&self, event: CreateBook) -> Result<(), String>;

    /// すべての本を返します。
    async fn find_all(&self) -> Result<Vec<String>, String>;

    /// 指定したIDの本を返します。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<book::Model>, String>;
}
