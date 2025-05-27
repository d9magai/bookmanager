use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Books::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("gen_random_uuid()".into())),
                    )
                    .col(ColumnDef::new(Books::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Books::Author).string_len(255).not_null())
                    .col(ColumnDef::new(Books::Isbn).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Books::Description)
                            .string_len(1024)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Books::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP(3)".into())),
                    )
                    .col(
                        ColumnDef::new(Books::UpdatedAt)
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
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    Title,
    Author,
    Isbn,
    Description,
    CreatedAt,
    UpdatedAt,
}
