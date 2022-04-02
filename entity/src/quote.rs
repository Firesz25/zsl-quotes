use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "quotes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub quote: String,
    pub author: String,
    pub category: String,
    pub create_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        panic!("not implemented")
    }
}

impl ActiveModelBehavior for ActiveModel {}
