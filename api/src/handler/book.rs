use axum::extract::{Json, State};
use axum::{http::StatusCode, response::IntoResponse};
use registry::AppRegistry;
use sea_orm::entity::prelude::*;
use thiserror::Error;

use crate::model::book::BookResponse;
use crate::model::book::CreateBookRequest;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(request): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(request.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(anyhow::Error::new)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<crate::model::book::BookResponse>>, AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|books| {
            Json(
                books
                    .into_iter()
                    .map(BookResponse::from)
                    .collect::<Vec<BookResponse>>(),
            )
        })
        .map_err(anyhow::Error::new)
        .map_err(AppError::from)
}

pub async fn show_book(
    Path(id): Path<Uuid>,
    State(registry): State<AppRegistry>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(id)
        .await
        .map(|book| Json(BookResponse::from(book)))
        .map_err(anyhow::Error::new)
        .map_err(AppError::from)
}
