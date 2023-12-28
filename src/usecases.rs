use sqlx::{Postgres, Transaction};

use crate::repositories::{SelectOneRepository, TransactionRepository, WithTransaction};

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
    ) -> WithTransaction<Result<i64, sqlx::Error>> {
        let select_one_repository = self.repositories.select_one_repository();
        let result1 = select_one_repository.select(tx).await;
        let result2 = select_one_repository.select(result1.tx).await;
        result2
    }

    pub async fn select_one(&self) -> Result<i64, sqlx::Error> {
        match self.repositories.transaction_repository().begin().await? {
            Some(tx) => {
                let result = self.inner(Some(tx)).await;
                result.tx.unwrap().commit().await?;
                result.value
            }
            None => self.inner(None).await.value,
        }
    }
}
