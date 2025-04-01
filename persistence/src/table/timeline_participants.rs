//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "match_v5", table_name = "timeline_participants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub puuid: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub match_id: String,
    pub participant_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::timelines::Entity",
        from = "Column::MatchId",
        to = "super::timelines::Column::MatchId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Timelines,
}

impl Related<super::timelines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Timelines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
