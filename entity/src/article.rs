//! `article.rs` - Defines the `Article` entity using `SeaORM`.
//! This module contains the `Model` struct representing an article,
//! along with its relations and behaviors.
//!
//! # Entity Overview
//! - Represents an article in the database.
//! - Includes fields such as `id`, `title`, `content`, `uuid`, `user_id`, etc.
//! - Establishes a relationship with the `User` entity.

use sea_orm::entity::prelude::*;

/// Represents an article in the database.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "article")]
pub struct Model {
    /// Unique identifier for the article (Primary Key).
    #[sea_orm(primary_key)]
    pub id: i32,
    
    /// Title of the article.
    pub title: String,
    
    /// The main content/body of the article.
    pub content: String,
    
    /// Universally Unique Identifier (UUID) for the article (Unique constraint).
    #[sea_orm(unique)]
    pub uuid: Uuid,
    
    /// ID of the user who created the article (Foreign Key).
    pub user_id: i32,
    
    /// Timestamp of when the article was created.
    pub created_at: DateTime,
    
    /// Optional field for storing an image URL or path associated with the article.
    pub image: Option<String>,
}

/// Defines relationships between `Article` and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship: Each article belongs to a single user.
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

/// Implements relationship behavior for `Article` and `User`.
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

/// Defines custom behavior for the active model (if needed in the future).
impl ActiveModelBehavior for ActiveModel {}
