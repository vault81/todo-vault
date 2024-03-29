use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Lists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Lists::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Lists::Title).string().not_null())
                    .clone(),
            )
            .await?;

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
                    .col(ColumnDef::new(Todos::ListId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todos::Table, Todos::ListId)
                            .to(Lists::Table, Lists::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Todos::Title).string().not_null())
                    .col(ColumnDef::new(Todos::Description).string())
                    .col(ColumnDef::new(Todos::Done).boolean().not_null())
                    .col(ColumnDef::new(Todos::DueDate).date())
                    .col(
                        ColumnDef::new(Todos::CreatedAt).date_time().not_null(),
                    )
                    .col(
                        ColumnDef::new(Todos::UpdatedAt).date_time().not_null(),
                    )
                    .clone(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).clone())
            .await?;

        manager
            .drop_table(Table::drop().table(Lists::Table).clone())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Todos {
    Table,
    Id,
    ListId,
    Title,
    Description,
    Done,
    DueDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Lists {
    Table,
    Id,
    Title,
}
