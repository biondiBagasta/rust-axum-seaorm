use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Username))
                    .col(string(User::Password))
                    .col(string(User::FullName))
                    .col(string(User::Address))
                    .col(string(User::PhoneNumber))
                    .col(string(User::Role))
                    .col(string(User::Photo))
                    .col(date_time(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(User::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    FullName,
    Address,
    PhoneNumber,
    Role,
    Photo,
    CreatedAt,
    UpdatedAt
}
