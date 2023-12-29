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

pub struct OnTransaction<T, E: std::error::Error> {
    pub value: Result<T, E>,
    pub tx: Option<Transaction<'static, Postgres>>,
}

#[derive(thiserror::Error, Debug)]
pub enum CommitErrorOr<E: std::error::Error> {
    CommitError(String),
    OtherError(E),
}

impl<T, E: std::error::Error> OnTransaction<T, E> {
    pub fn new(value: Result<T, E>, tx: Option<Transaction<'static, Postgres>>) -> Self {
        OnTransaction { value, tx }
    }
    pub async fn and_then_commit(self) -> Result<T, CommitErrorOr<E>> {
        match self.value {
            Ok(value) => {
                if let Some(tx) = self.tx {
                    tx.commit()
                        .await
                        .map_err(|e| CommitErrorOr::CommitError(e.to_string()))?;
                }
                Ok(value)
            }
            Err(e) => Err(CommitErrorOr::OtherError(e)),
        }
    }
}

#[mockall::automock]
#[async_trait]
pub(crate) trait SelectOneRepository {
    async fn select(
        &self,
        tx: Option<Transaction<'static, Postgres>>,
    ) -> OnTransaction<i64, sqlx::Error>;
}

pub(crate) struct SelectOneRepositoryImpl();

#[async_trait]
impl SelectOneRepository for SelectOneRepositoryImpl {
    async fn select(
        &self,
        tx: Option<Transaction<'static, Postgres>>,
    ) -> OnTransaction<i64, sqlx::Error> {
        let stmt = "select 1";
        let mut tx = tx.unwrap();
        let result = query_scalar::<_, i64>(stmt).fetch_one(tx.as_mut()).await;
        OnTransaction::new(result, Some(tx))
    }
}
