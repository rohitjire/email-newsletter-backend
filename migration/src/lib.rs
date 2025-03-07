//! `lib.rs` - Defines the database migrations using `SeaORM`.
//! This module manages database schema changes through migrations.
//!
//! # Migrations
//! - `m20241130_145647_create_user_table` - Creates the `User` table.
//! - `m20250102_221835_article_table` - Creates the `Article` table.
//! - `m20250208_132108_subscription_table` - Creates the `Subscription` table.

pub use sea_orm_migration::prelude::*;

//mod m20220101_000001_create_table;
mod m20241130_145647_create_user_table;
mod m20250102_221835_article_table;
mod m20250208_132108_subscription_table;

/// Handles database migrations.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// Registers all migrations in the correct order.
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241130_145647_create_user_table::Migration),
            Box::new(m20250102_221835_article_table::Migration),
            Box::new(m20250208_132108_subscription_table::Migration)
        ]
    }
}
