use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // どのバックエンドか取得
        let db_backend = manager.get_database_backend();
        let id_column = if db_backend == DbBackend::Sqlite {
            ColumnDef::new(Books::Id)
                .string_len(36)
                .not_null()
                .primary_key()
                .to_owned()
        } else {
            ColumnDef::new(Books::Id)
                .uuid()
                .not_null()
                .primary_key()
                .default(SimpleExpr::Custom("gen_random_uuid()".into()))
                .to_owned()
        };

        let created_at_column = if db_backend == DbBackend::Sqlite {
            // SQLite: DATETIME型にして CURRENT_TIMESTAMP を使う
            ColumnDef::new(Books::CreatedAt)
                .timestamp() // SQLiteでは TIMESTAMP == DATETIME
                .not_null()
                // SQLiteでは精度指定なしの CURRENT_TIMESTAMP にする
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_owned()))
                .to_owned()
        } else {
            // Postgres: 本来の TIMESTAMP WITH TIME ZONE(3) と CURRENT_TIMESTAMP(3)
            ColumnDef::new(Books::CreatedAt)
                .timestamp_with_time_zone()
                .not_null()
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP(3)".to_owned()))
                .to_owned()
        };

        let updated_at_column = if db_backend == DbBackend::Sqlite {
            // SQLite: DATETIME型にして CURRENT_TIMESTAMP を使う
            ColumnDef::new(Books::UpdatedAt)
                .timestamp() // SQLiteでは TIMESTAMP == DATETIME
                .not_null()
                // SQLiteでは精度指定なしの CURRENT_TIMESTAMP にする
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_owned()))
                .to_owned()
        } else {
            // Postgres: 本来の TIMESTAMP WITH TIME ZONE(3) と CURRENT_TIMESTAMP(3)
            ColumnDef::new(Books::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null()
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP(3)".to_owned()))
                .to_owned()
        };

        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(id_column)
                    .col(ColumnDef::new(Books::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Books::Author).string_len(255).not_null())
                    .col(ColumnDef::new(Books::Isbn).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Books::Description)
                            .string_len(1024)
                            .not_null(),
                    )
                    .col(created_at_column)
                    .col(updated_at_column)
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
