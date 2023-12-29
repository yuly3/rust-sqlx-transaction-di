use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query_scalar, Pool, Postgres, Transaction};

#[mockall::automock]
#[async_trait]
pub(crate) trait TransactionRepository {
    async fn begin(&self) -> Result<Option<Transaction<'static, Postgres>>, sqlx::Error>;
}

pub(crate) struct TransactionRepositoryImpl(pub(crate) Arc<Pool<Postgres>>);

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn begin(&self) -> Result<Option<Transaction<'static, Postgres>>, sqlx::Error> {
        Ok(Some(self.0.begin().await?))
    }
}

pub struct OnTransaction<T> {
    pub value: T,
    pub tx: Option<Transaction<'static, Postgres>>,
}

impl<T> OnTransaction<T> {
    pub fn new(value: T, tx: Option<Transaction<'static, Postgres>>) -> Self {
        OnTransaction { value, tx }
    }
    pub async fn commit(self) -> Result<T, sqlx::Error> {
        self.tx.unwrap().commit().await.map(|_| self.value)
    }
}

#[mockall::automock]
#[async_trait]
pub(crate) trait SelectOneRepository {
    async fn select<'a>(
        &self,
        tx: Option<Transaction<'static, Postgres>>,
    ) -> OnTransaction<Result<i64, sqlx::Error>>;
}

pub(crate) struct SelectOneRepositoryImpl();

#[async_trait]
impl SelectOneRepository for SelectOneRepositoryImpl {
    async fn select<'a>(
        &self,
        tx: Option<Transaction<'static, Postgres>>,
    ) -> OnTransaction<Result<i64, sqlx::Error>> {
        let stmt = "select 1";
        let mut tx = tx.unwrap();
        let result = query_scalar::<_, i64>(stmt).fetch_one(tx.as_mut()).await;
        OnTransaction::new(result, Some(tx))
    }
}
