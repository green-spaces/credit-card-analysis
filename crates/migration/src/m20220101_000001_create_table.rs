use sea_orm_migration::{prelude::*, sea_query::Iden};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        create_raw_csv_table(manager).await?;
        create_bill_line_description_table(manager).await?;
        create_bill_line_table(manager).await?;
        create_category_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        drop_bill_line_table(manager).await?;
        drop_bill_line_description_table(manager).await?;
        drop_category_table(manager).await?;
        drop_csv_table(manager).await
    }
}

async fn create_raw_csv_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                .col(
                    ColumnDef::new(RawCsv::Text)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .to_owned(),
        )
        .await
}

async fn drop_csv_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(sea_query::Table::drop().table(RawCsv::Table).to_owned())
        .await
}

async fn create_bill_line_description_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            sea_query::Table::create()
                .table(BillLineDescription::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(BillLineDescription::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment(),
                )
                .col(
                    ColumnDef::new(BillLineDescription::Description)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(BillLineDescription::DescriptionCategoryId).integer())
                .foreign_key(
                    sea_query::ForeignKey::create()
                        .name("FK_category")
                        .from(
                            BillLineDescription::Table,
                            BillLineDescription::DescriptionCategoryId,
                        )
                        .to(DescriptionCategory::Table, DescriptionCategory::Id),
                )
                .to_owned(),
        )
        .await
}

async fn drop_bill_line_description_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(
            sea_query::Table::drop()
                .table(BillLineDescription::Table)
                .to_owned(),
        )
        .await
}

async fn create_bill_line_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            sea_query::Table::create()
                .table(BillLine::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(BillLine::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment(),
                )
                .col(ColumnDef::new(BillLine::TransactionData).date().not_null())
                .col(ColumnDef::new(BillLine::DescriptionId).integer().not_null())
                .col(ColumnDef::new(BillLine::Debit).decimal())
                .col(ColumnDef::new(BillLine::Credit).decimal())
                .col(ColumnDef::new(BillLine::Balance).decimal().not_null())
                .col(ColumnDef::new(BillLine::RawCsvId).integer().not_null())
                .foreign_key(
                    sea_query::ForeignKey::create()
                        .name("FK_raw_csv")
                        .from(BillLine::Table, BillLine::RawCsvId)
                        .to(RawCsv::Table, RawCsv::Id),
                )
                .foreign_key(
                    sea_query::ForeignKey::create()
                        .name("FK_description")
                        .from(BillLine::Table, BillLine::DescriptionId)
                        .to(BillLineDescription::Table, BillLineDescription::Id),
                )
                .to_owned(),
        )
        .await
}

async fn drop_bill_line_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(sea_query::Table::drop().table(BillLine::Table).to_owned())
        .await
}

async fn create_category_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            sea_query::Table::create()
                .table(DescriptionCategory::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DescriptionCategory::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(DescriptionCategory::Name)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .to_owned(),
        )
        .await
}

async fn drop_category_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(
            sea_query::Table::drop()
                .table(DescriptionCategory::Table)
                .to_owned(),
        )
        .await
}

#[derive(Iden)]
enum RawCsv {
    Table,
    Id,
    Text,
}

#[derive(Iden)]
enum BillLine {
    Table,
    Id,
    TransactionData,
    DescriptionId,
    Debit,
    Credit,
    Balance,
    RawCsvId,
}

#[derive(Iden)]
enum BillLineDescription {
    Table,
    Id,
    Description,
    DescriptionCategoryId,
}

#[derive(Iden)]
enum DescriptionCategory {
    Table,
    Id,
    Name,
}
