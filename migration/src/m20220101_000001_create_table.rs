/// Migration script for creating the `Post` table.
/// This migration uses `sea_orm_migration` to define the schema.
use sea_orm_migration::{prelude::*, schema::*};

/// Struct representing the migration.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Runs the `up` migration to create the `Post` table.
    /// 
    /// # Arguments
    /// * `manager` - The schema manager to handle database operations.
    /// 
    /// # Errors
    /// Returns `DbErr` if the creation fails.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Replace with your migration logic
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id)) // Primary key column with auto-increment
                    .col(string(Post::Title)) // Title column of type String
                    .col(string(Post::Text)) // Text column of type String
                    .to_owned(),
            )
            .await
    }

    /// Runs the `down` migration to drop the `Post` table.
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
                    .table(Post::Table) // Drops the `Post` table
                    .to_owned(),
            )
            .await
    }
}

/// Enum representing identifiers (columns and table name) for `Post`.
#[derive(DeriveIden)]
enum Post {
    /// Table identifier for `Post`
    Table,
    /// Column identifier for `id`
    Id,
    /// Column identifier for `title`
    Title,
    /// Column identifier for `text`
    Text,
}
