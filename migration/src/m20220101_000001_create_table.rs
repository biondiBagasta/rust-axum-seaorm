use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Id))
                    .col(string(Category::Name))
                    .col(date_time(Category::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Category::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await    
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt
}

