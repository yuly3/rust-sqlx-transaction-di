use sqlx::{Postgres, Transaction};

use crate::repositories::{SelectOneRepository, TransactionRepository};

pub(crate) trait TransactionExt {
    type TransactionRepo: TransactionRepository;
    fn transaction_repository(&self) -> &Self::TransactionRepo;
}

pub(crate) trait SelectOneExt: TransactionExt {
    type SelectOneRepo: SelectOneRepository;
    fn select_one_repository(&self) -> &Self::SelectOneRepo;
}

pub(crate) struct SelectOneUseCase<S: SelectOneExt> {
    repositories: S,
}

impl<S: SelectOneExt> SelectOneUseCase<S> {
    pub fn new(repositories: S) -> Self {
        Self { repositories }
    }

    async fn inner(
        &self,
        tx: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<i64, sqlx::Error> {
        self.repositories.select_one_repository().select(tx).await
    }

    pub async fn select_one(&self) -> Result<i64, sqlx::Error> {
        match self.repositories.transaction_repository().begin().await? {
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
