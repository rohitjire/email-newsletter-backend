/// Migration script for creating the `Subscription` table.
/// This migration uses `sea_orm_migration` and references the `User` table for foreign keys.
use sea_orm_migration::prelude::*;
use crate::m20241130_145647_create_user_table::User;

/// Struct representing the migration.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Runs the `up` migration to create the `Subscription` table.
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
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subscription::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Subscription::SubscribedUserId).integer().not_null())
                    .col(ColumnDef::new(Subscription::SubscriberUserId).integer().not_null())
                    .col(ColumnDef::new(Subscription::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-subscription-user_id")
                            .from(Subscription::Table, Subscription::SubscribedUserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-subscription-subscriber_id")
                            .from(Subscription::Table, Subscription::SubscriberUserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    /// Runs the `down` migration to drop the `Subscription` table.
    /// 
    /// # Arguments
    /// * `manager` - The schema manager to handle database operations.
    /// 
    /// # Errors
    /// Returns `DbErr` if the drop fails.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

/// Enum representing identifiers (columns and table name) for `Subscription`.
#[derive(DeriveIden)]
pub enum Subscription {
    /// Table identifier for `Subscription`
    Table,
    /// Column identifier for `id`
    Id,
    /// Column identifier for `subscribed_user_id`
    SubscribedUserId,
    /// Column identifier for `subscriber_user_id`
    SubscriberUserId,
    /// Column identifier for `created_at`
    CreatedAt,
}
