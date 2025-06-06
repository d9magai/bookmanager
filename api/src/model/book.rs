use kernel::{entity::books, model::book::event::CreateBook};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(request: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = request;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
impl From<books::Model> for BookResponse {
    fn from(book: books::Model) -> Self {
        Self {
            id: book.id.to_string(),
            title: book.title,
            author: book.author,
            isbn: book.isbn,
            description: book.description,
        }
    }
}
