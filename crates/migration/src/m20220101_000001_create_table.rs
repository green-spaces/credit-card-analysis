use sea_orm_migration::{prelude::*, sea_query::Iden};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                sea_query::Table::create()
                    .table(RawCsv::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RawCsv::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RawCsv::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(sea_query::Table::drop().table(RawCsv::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RawCsv {
    Table,
    Id,
    Text,
}
