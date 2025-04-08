pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250331_090647_add_product_table;
mod m20250401_083816_create_table_product;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250331_090647_add_product_table::Migration),
            Box::new(m20250401_083816_create_table_product::Migration),
        ]
    }
}
