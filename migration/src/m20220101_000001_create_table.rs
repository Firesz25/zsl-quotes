use entity::prelude::*;
use sea_schema::migration::{
    sea_orm::{DbBackend, EntityTrait, Schema},
    sea_query::*,
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![create_seorm_table(Quote)];
        for stmt in stmts {
            manager.create_table(stmt).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![drop_seorm_table(Quote)];
        for stmt in stmts {
            manager.drop_table(stmt).await?;
        }
        Ok(())
    }
}

fn create_seorm_table(e: impl EntityTrait) -> TableCreateStatement {
    let scheme = Schema::new(DbBackend::MySql);
    scheme
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}

fn drop_seorm_table(e: impl EntityTrait) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}