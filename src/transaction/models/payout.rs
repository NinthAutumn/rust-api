use tokio_pg_mapper_derive::PostgresMapper;
use super::transaction::Transaction;
use futures::Future;
#[derive(PostgresMapper)]
#[pg_mapper(table = "payout_transactions")]
pub struct PayoutTransaction {
    charge: i32,
    currency: String,
    stripe_payout_id: String,
    transaction: dyn Future<Transaction>,
    created_at: String // Should be date?
}
