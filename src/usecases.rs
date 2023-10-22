use sqlx::{Postgres, Transaction};

use crate::{SelectOneRepository, TransactionRepository};

pub(crate) struct SelectOneUseCase<S: SelectOneRepository, T: for<'a> TransactionRepository<'a>> {
    select_repository: S,
    transaction_repository: T,
}

impl<S: SelectOneRepository, T: for<'a> TransactionRepository<'a>> SelectOneUseCase<S, T> {
    pub fn new(select_repository: S, transaction_repository: T) -> Self {
        Self {
            select_repository,
            transaction_repository,
        }
    }

    async fn inner(
        &self,
        tx: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error> {
        self.select_repository.select(tx).await
    }

    pub async fn select_one(&self) -> Result<i64, sqlx::Error> {
        match self.transaction_repository.begin().await? {
            Some(mut tx) => {
                let result = {
                    let tx = Some(&mut tx);
                    self.inner(tx).await
                };
                tx.commit().await?;
                result
            }
            None => self.inner(None).await,
        }
    }
}
