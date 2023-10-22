use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query_scalar, Pool, Postgres, Transaction};

#[async_trait]
pub(crate) trait TransactionRepository<'a> {
    async fn begin(&self) -> Result<Option<Transaction<'a, Postgres>>, sqlx::Error>;
}

pub(crate) struct TransactionRepositoryImpl(pub(crate) Arc<Pool<Postgres>>);

#[async_trait]
impl<'a> TransactionRepository<'a> for TransactionRepositoryImpl {
    async fn begin(&self) -> Result<Option<Transaction<'a, Postgres>>, sqlx::Error> {
        Ok(Some(self.0.begin().await?))
    }
}

#[async_trait]
pub(crate) trait SelectOneRepository {
    async fn select(
        &self,
        tx: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error>;
}

pub(crate) struct SelectRepositoryImpl();

#[async_trait]
impl SelectOneRepository for SelectRepositoryImpl {
    async fn select(
        &self,
        tx: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error> {
        let stmt = "select 1";
        query_scalar::<_, i64>(stmt)
            .fetch_one(tx.unwrap().as_mut())
            .await
    }
}
