//! `user.rs` - Defines the `User` entity using `SeaORM`.
//! This module represents a user in the system.
//!
//! # Entity Overview
//! - Represents a user in the database.
//! - Contains fields such as `id`, `name`, `email`, and `password`.
//! - Establishes a one-to-many relationship with the `Article` entity.

use sea_orm::entity::prelude::*;

/// Represents a user in the database.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    /// Unique identifier for the user (Primary Key).
    #[sea_orm(primary_key)]
    pub id: i32,
    
    /// Full name of the user.
    pub name: String,
    
    /// Unique email address of the user.
    #[sea_orm(unique)]
    pub email: String,
    
    /// Hashed password of the user.
    pub password: String,
}

/// Defines relationships between `User` and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship: A user can have multiple articles.
    #[sea_orm(has_many = "super::article::Entity")]
    Article,
}

/// Implements relationship behavior for `User` and `Article`.
impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

/// Defines custom behavior for the active model (if needed in the future).
impl ActiveModelBehavior for ActiveModel {}
