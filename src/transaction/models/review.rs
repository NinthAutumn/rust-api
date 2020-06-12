use tokio_pg_mapper_derive::PostgresMapper;
use super::transaction::Transaction;
use futures::Future;
#[derive(PostgresMapper)]
#[pg_mapper(table = "review_transactions")]
pub struct ReviewTransaction {
    review_id: i32,
    review: dyn Future<>, // dyn Future<Review> Haven't made review yet
    transaction_id: i32,
    transaction: dyn Future<Transaction>
}