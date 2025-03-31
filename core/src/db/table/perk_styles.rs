//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "match_v5", table_name = "perk_styles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub perk_style_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub match_id: String,
    #[sea_orm(column_type = "Text")]
    pub puuid: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub style: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::participant_perks::Entity",
        from = "(Column::MatchId, Column::Puuid)",
        to = "(super::participant_perks::Column::MatchId, super::participant_perks::Column::Puuid)",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ParticipantPerks,
    #[sea_orm(has_many = "super::perk_style_selections::Entity")]
    PerkStyleSelections,
}

impl Related<super::participant_perks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParticipantPerks.def()
    }
}

impl Related<super::perk_style_selections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PerkStyleSelections.def()
    }
}

impl Related<super::participants::Entity> for Entity {
    fn to() -> RelationDef {
        super::participant_perks::Relation::Participants.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::participant_perks::Relation::PerkStyles.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
