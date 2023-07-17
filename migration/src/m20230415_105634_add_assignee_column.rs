use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todos::Table)
                    .add_column(ColumnDef::new(Todos::Assignee).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todos::Table)
                    .drop_column(Todos::Assignee)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Todos {
    Table,
    Assignee,
}
