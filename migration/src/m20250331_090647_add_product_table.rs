use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(string(Product::Name))
                    .col(text(Product::Description))
                    .col(integer(Product::PurchasePrice))
                    .col(integer(Product::SellingPrice))
                    .col(integer(Product::Stock))
                    .col(integer(Product::Discount))
                    .col(string(Product::Image))
                    .col(integer(Product::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_product_category")
                        .from(Product::Table, Product::CategoryId)
                        .to(Category::Table, Category::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(date_time(Product::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Product::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    PurchasePrice,
    SellingPrice,
    Stock,
    Discount,
    Image,
    CategoryId,
    CreatedAt,
    UpdatedAt
}