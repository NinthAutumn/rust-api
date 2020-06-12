use tokio_pg_mapper_derive::PostgresMapper;
use crate::user::models::user::User;
use futures::Future;
#[derive(PostgresMapper)]
#[pg_mapper(table = "transactions")]
pub struct Transaction {
    id: i32,
    amount: i32,
    sender_id: i32,
    receiver_id: i32,
    sender: dyn Future<User>,
    receiver: dyn Future<User>,
    redeemed: bool,
    created_at: String // Should be date?
}