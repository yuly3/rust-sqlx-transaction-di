use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query_scalar, Pool, Postgres, Transaction};

#[mockall::automock]
#[async_trait]
pub(crate) trait TransactionRepository {
    async fn begin<'a>(&self) -> Result<Option<Transaction<'a, Postgres>>, sqlx::Error>;
}

pub(crate) struct TransactionRepositoryImpl(pub(crate) Arc<Pool<Postgres>>);

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn begin<'a>(&self) -> Result<Option<Transaction<'a, Postgres>>, sqlx::Error> {
        Ok(Some(self.0.begin().await?))
    }
}

#[mockall::automock]
#[async_trait]
pub(crate) trait SelectOneRepository {
    async fn select<'a>(
        &self,
        tx: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error>;
}

pub(crate) struct SelectOneRepositoryImpl();

#[async_trait]
impl SelectOneRepository for SelectOneRepositoryImpl {
    async fn select<'a>(
        &self,
        tx: Option<&'a mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error> {
        let stmt = "select 1";
        query_scalar::<_, i64>(stmt)
            .fetch_one(tx.unwrap().as_mut())
            .await
    }
}
