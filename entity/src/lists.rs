use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Hash,
)]
#[sea_orm(table_name = "lists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id:    Uuid,
    pub title: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::todos::Entity")]
    Todos,
}

impl Related<super::todos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todos.def()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            id:    uuid::Uuid::new_v4(),
            title: "default".to_string(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(title: String) -> Self {
        Self {
            id:    Set(uuid::Uuid::new_v4()),
            title: Set(title),
        }
    }
}
