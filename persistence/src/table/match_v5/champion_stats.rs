//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "match_v5", table_name = "champion_stats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub frame_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_id: i32,
    pub ability_haste: Option<i32>,
    pub ability_power: i32,
    pub armor: i32,
    pub armor_pen: i32,
    pub armor_pen_percent: i32,
    pub attack_damage: i32,
    pub attack_speed: i32,
    pub bonus_armor_pen_percent: i32,
    pub bonus_magic_pen_percent: i32,
    pub cc_reduction: i32,
    pub cooldown_reduction: i32,
    pub health: i32,
    pub health_max: i32,
    pub health_regen: i32,
    pub lifesteal: i32,
    pub magic_pen: i32,
    pub magic_pen_percent: i32,
    pub magic_resist: i32,
    pub movement_speed: i32,
    pub omnivamp: Option<i32>,
    pub physical_vamp: Option<i32>,
    pub power: i32,
    pub power_max: i32,
    pub power_regen: i32,
    pub spell_vamp: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::timeline_participant_frames::Entity",
        from = "(Column::FrameId, Column::ParticipantId)",
        to = "(super::timeline_participant_frames::Column::FrameId, super::timeline_participant_frames::Column::ParticipantId)",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TimelineParticipantFrames,
}

impl Related<super::timeline_participant_frames::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimelineParticipantFrames.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
