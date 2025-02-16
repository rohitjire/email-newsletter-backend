/// Migration script for creating the `User` table.
/// This migration uses `sea_orm_migration` to define the user schema.
use sea_orm_migration::prelude::*;

/// Struct representing the migration.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Runs the `up` migration to create the `User` table.
    /// 
    /// # Arguments
    /// * `manager` - The schema manager to handle database operations.
    /// 
    /// # Errors
    /// Returns `DbErr` if the creation fails.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(), // User ID (Primary Key, Auto-increment)
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string()
                            .not_null(), // User name (String, required)
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key(), // Email (Unique, required)
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null(), // Password (String, required)
                    )
                    .to_owned(),
            )
            .await
    }

    /// Runs the `down` migration to drop the `User` table.
    /// 
    /// # Arguments
    /// * `manager` - The schema manager to handle database operations.
    /// 
    /// # Errors
    /// Returns `DbErr` if the drop fails.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(User::Table) // Drops the `User` table
                    .to_owned(),
            )
            .await
    }
}

/// Enum representing identifiers (columns and table name) for `User`.
#[derive(DeriveIden)]
pub enum User {
    /// Table identifier for `User`
    Table,
    /// Column identifier for `id`
    Id,
    /// Column identifier for `name`
    Name,
    /// Column identifier for `email`
    Email,
    /// Column identifier for `password`
    Password,
}
