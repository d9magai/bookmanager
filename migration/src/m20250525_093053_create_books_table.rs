use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Book::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("gen_random_uuid()".into())),
                    )
                    .col(ColumnDef::new(Book::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Book::Author).string_len(255).not_null())
                    .col(ColumnDef::new(Book::Isbn).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Book::Description)
                            .string_len(1024)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Book::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP(3)".into())),
                    )
                    .col(
                        ColumnDef::new(Book::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP(3)".into())),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    Title,
    Author,
    Isbn,
    Description,
    CreatedAt,
    UpdatedAt,
}
