#![forbid(unsafe_code)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_todos_table;
mod m20230415_105634_add_assignee_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_todos_table::Migration),
            Box::new(m20230415_105634_add_assignee_column::Migration),
        ]
    }
}
