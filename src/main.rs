use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use crate::repositories::SelectOneRepositoryImpl;
use crate::repositories::TransactionRepositoryImpl;
use crate::usecases::{SelectOneExt, TransactionExt, UseCaseOnTransaction};

mod repositories;
mod usecases;

struct SelectOneRepositories {
    transaction_repository: TransactionRepositoryImpl,
    select_one_repository: SelectOneRepositoryImpl,
}

impl SelectOneRepositories {
    pub fn new(
        transaction_repository: TransactionRepositoryImpl,
        select_one_repository: SelectOneRepositoryImpl,
    ) -> Self {
        SelectOneRepositories {
            transaction_repository,
            select_one_repository,
        }
    }
}

impl TransactionExt for SelectOneRepositories {
    type TransactionRepo = TransactionRepositoryImpl;
    fn transaction_repository(&self) -> &Self::TransactionRepo {
        &self.transaction_repository
    }
}

impl SelectOneExt for SelectOneRepositories {
    type SelectOneRepo = SelectOneRepositoryImpl;
    fn select_one_repository(&self) -> &Self::SelectOneRepo {
        &self.select_one_repository
    }
}

#[tokio::main]
async fn main() {
    let pool = Arc::new(PgPoolOptions::new().connect("localhost").await.unwrap());

    let transaction_repository = TransactionRepositoryImpl(pool);
    let select_one_repository = SelectOneRepositoryImpl();
    let select_one_repositories =
        SelectOneRepositories::new(transaction_repository, select_one_repository);

    let select_one_usecase = UseCaseOnTransaction::new(select_one_repositories);
    let result = select_one_usecase.select_one().await.unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::repositories::{MockSelectOneRepository, MockTransactionRepository, OnTransaction};
    use crate::usecases::{SelectOneExt, TransactionExt, UseCaseOnTransaction};

    struct SelectOneRepositories {
        transaction_repository: MockTransactionRepository,
        select_one_repository: MockSelectOneRepository,
    }

    impl TransactionExt for SelectOneRepositories {
        type TransactionRepo = MockTransactionRepository;
        fn transaction_repository(&self) -> &Self::TransactionRepo {
            &self.transaction_repository
        }
    }

    impl SelectOneExt for SelectOneRepositories {
        type SelectOneRepo = MockSelectOneRepository;
        fn select_one_repository(&self) -> &Self::SelectOneRepo {
            &self.select_one_repository
        }
    }

    #[tokio::test]
    async fn test() {
        let mut select_one_repository = MockSelectOneRepository::new();
        select_one_repository
            .expect_select()
            .returning(|_tx| OnTransaction::new(Ok(1), None));

        let mut transaction_repository = MockTransactionRepository::new();
        transaction_repository.expect_begin().returning(|| Ok(None));

        let select_one_repositories = SelectOneRepositories {
            transaction_repository,
            select_one_repository,
        };

        let result = UseCaseOnTransaction::new(select_one_repositories)
            .select_one()
            .await;
        assert_eq!(result.unwrap(), 1);
    }
}
