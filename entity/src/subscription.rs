//! `subscription.rs` - Defines the `Subscription` entity using `SeaORM`.
//! This module represents the subscriptions between users.
//!
//! # Entity Overview
//! - Represents a subscription relationship between users.
//! - Contains fields such as `id`, `subscribed_user_id`, `subscriber_user_id`, and `created_at`.
//! - Establishes relationships with the `User` entity.

use sea_orm::entity::prelude::*;

/// Represents a subscription relationship between users.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "subscription")]
pub struct Model {
    /// Unique identifier for the subscription (Primary Key).
    #[sea_orm(primary_key)]
    pub id: i32,
    
    /// ID of the user being subscribed to.
    pub subscribed_user_id: i32,
    
    /// ID of the user who is subscribing.
    pub subscriber_user_id: i32,
    
    /// Timestamp of when the subscription was created.
    pub created_at: DateTime,
}

/// Defines relationships between `Subscription` and `User` entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship: Subscription belongs to a subscribing user.
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SubscriberUserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"    )]
    User2,
    
    /// Relationship: Subscription belongs to a subscribed user.
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SubscribedUserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User1,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User1.def()
    }
}

/// Defines custom behavior for the active model (if needed in the future).
impl ActiveModelBehavior for ActiveModel {}
