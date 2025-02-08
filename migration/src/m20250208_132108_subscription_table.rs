use sea_orm_migration::prelude::*;
use crate::m20241130_145647_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscription::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Subscription {
    Table,
    Id,
    SubscribedUserId,
    SubscriberUserId,
    CreatedAt,
}
