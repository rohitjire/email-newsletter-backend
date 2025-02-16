/// Migration script for creating the `Article` table.
/// This migration uses `sea_orm_migration` and references the `User` table.
use sea_orm_migration::prelude::*;
use crate::m20241130_145647_create_user_table::User;

/// Struct representing the migration.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Runs the `up` migration to create the `Article` table.
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
                    .table(Article::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Article::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::Content).string().not_null())
                    .col(ColumnDef::new(Article::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Article::UserId).integer().not_null())
                    .col(ColumnDef::new(Article::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Article::Image).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_user_id")
                            .from(Article::Table, Article::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    /// Runs the `down` migration to drop the `Article` table.
    /// 
    /// # Arguments
    /// * `manager` - The schema manager to handle database operations.
    /// 
    /// # Errors
    /// Returns `DbErr` if the drop fails.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

/// Enum representing identifiers (columns and table name) for `Article`.
#[derive(DeriveIden)]
pub enum Article {
    /// Table identifier for `Article`
    Table,
    /// Column identifier for `id`
    Id,
    /// Column identifier for `title`
    Title,
    /// Column identifier for `content`
    Content,
    /// Column identifier for `uuid`
    Uuid,
    /// Column identifier for `user_id`
    UserId,
    /// Column identifier for `created_at`
    CreatedAt,
    /// Column identifier for `image`
    Image,
}
