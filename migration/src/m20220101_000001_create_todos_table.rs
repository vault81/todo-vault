use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todos::Title).string().not_null())
                    .col(ColumnDef::new(Todos::Text).string())
                    .col(ColumnDef::new(Todos::Done).boolean().not_null())
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).clone())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Todos {
    Table,
    Id,
    Title,
    Text,
    Done,
}
