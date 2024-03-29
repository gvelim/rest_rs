//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub author_id: i32,
    pub title: String,
    pub cover: String,
    pub year: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Id",
        to = "Column::AuthorId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef2,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Id",
        to = "Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef1,
}

impl ActiveModelBehavior for ActiveModel {}
