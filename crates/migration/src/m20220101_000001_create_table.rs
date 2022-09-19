use sea_orm_migration::{prelude::*, sea_query::Iden};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        create_raw_csv_table(manager).await?;
        create_bill_line_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        drop_csv_table(manager).await?;
        drop_bill_line_table(manager).await
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
                .col(ColumnDef::new(RawCsv::Text).string().not_null())
                .to_owned(),
        )
        .await
}

async fn drop_csv_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(sea_query::Table::drop().table(RawCsv::Table).to_owned())
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
                .col(ColumnDef::new(BillLine::Description).string().not_null())
                .col(ColumnDef::new(BillLine::Debit).decimal())
                .col(ColumnDef::new(BillLine::Credit).decimal())
                .col(ColumnDef::new(BillLine::Balance).decimal().not_null())
                .to_owned(),
        )
        .await
}

async fn drop_bill_line_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(sea_query::Table::drop().table(BillLine::Table).to_owned())
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
    Description,
    Debit,
    Credit,
    Balance,
    RawCsvId,
}

// transaction_data: TdDate,
// description: String,
// debit: Option<Money>,
// credit: Option<Money>,
// balance: Money,
