use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use crate::repositories::{SelectOneRepository, SelectRepositoryImpl};
use crate::repositories::{TransactionRepository, TransactionRepositoryImpl};
use crate::usecases::SelectOneUseCase;

mod repositories;
mod usecases;

#[tokio::main]
async fn main() {
    let pool = Arc::new(PgPoolOptions::new().connect("localhost").await.unwrap());
    let transaction_repository = TransactionRepositoryImpl(pool);
    let select_repository = SelectRepositoryImpl();
    let usecase = SelectOneUseCase::new(select_repository, transaction_repository);
    let result = usecase.select_one().await.unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use sqlx::{Postgres, Transaction};

    use super::*;

    #[tokio::test]
    async fn test() {
        struct MockSelectOneRepositoryImpl();

        #[async_trait]
        impl SelectOneRepository for MockSelectOneRepositoryImpl {
            async fn select(
                &self,
                _tx: Option<&mut Transaction<'static, Postgres>>,
            ) -> Result<i64, sqlx::Error> {
                Ok(1)
            }
        }

        struct MockTransactionRepositoryImpl();

        #[async_trait]
        impl<'a> TransactionRepository<'a> for MockTransactionRepositoryImpl {
            async fn begin(&self) -> Result<Option<Transaction<'a, Postgres>>, sqlx::Error> {
                Ok(None)
            }
        }

        let select_repository = MockSelectOneRepositoryImpl();
        let transaction_repository = MockTransactionRepositoryImpl();
        let result = SelectOneUseCase::new(select_repository, transaction_repository)
            .select_one()
            .await;
        assert_eq!(result.unwrap(), 1);
    }
}
