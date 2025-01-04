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
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Article {
    Table,
    Id,
    Title,
    Content,
    Uuid,
    UserId,
    Image,
    CreatedAt
}
