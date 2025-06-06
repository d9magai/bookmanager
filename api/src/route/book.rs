use crate::handler::book::register_book;
use crate::handler::book::{show_book, show_book_list};
use axum::{
    Router,
    routing::{get, post},
};
use registry::AppRegistry;

pub fn build_book_routers() -> Router<AppRegistry> {
    let books_routes = Router::new()
        .route("/", post(register_book).get(show_book_list))
        .route("/{id}", get(show_book));

    Router::new().nest("/books", books_routes)
}
