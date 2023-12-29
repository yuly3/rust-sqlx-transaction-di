use sqlx::{Postgres, Transaction};

use crate::repositories::{OnTransaction, SelectOneRepository, TransactionRepository};

pub(crate) trait TransactionExt {
    type TransactionRepo: TransactionRepository;
    fn transaction_repository(&self) -> &Self::TransactionRepo;
}

pub(crate) struct UseCaseOnTransaction<R: TransactionExt> {
    repositories: R,
}

impl<R: TransactionExt> UseCaseOnTransaction<R> {
    pub fn new(repositories: R) -> Self {
        Self { repositories }
    }
}

pub(crate) trait SelectOneExt: TransactionExt {
    type SelectOneRepo: SelectOneRepository;
    fn select_one_repository(&self) -> &Self::SelectOneRepo;
}

impl<R: SelectOneExt> UseCaseOnTransaction<R> {
    async fn inner(
        &self,
        tx: Option<Transaction<'static, Postgres>>,
    ) -> OnTransaction<Result<i64, sqlx::Error>> {
        let select_one_repository = self.repositories.select_one_repository();
        let result1 = select_one_repository.select(tx).await;
        select_one_repository.select(result1.tx).await
    }

    pub async fn select_one(&self) -> Result<i64, sqlx::Error> {
        match self.repositories.transaction_repository().begin().await? {
            Some(tx) => self.inner(Some(tx)).await.commit().await?,
            None => self.inner(None).await.value,
        }
    }
}
