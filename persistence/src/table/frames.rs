//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "match_v5", table_name = "frames")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub frame_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub match_id: String,
    pub timestamp: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::timeline_participant_frames::Entity")]
    TimelineParticipantFrames,
    #[sea_orm(
        belongs_to = "super::timelines::Entity",
        from = "Column::MatchId",
        to = "super::timelines::Column::MatchId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Timelines,
}

impl Related<super::timeline_participant_frames::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimelineParticipantFrames.def()
    }
}

impl Related<super::timelines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Timelines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
