use tokio_pg_mapper_derive::PostgresMapper;
use super::user::User;
use futures::Future;
#[derive(PostgresMapper)]
#[pg_mapper(table = "payout_methods")]
pub enum PlatformType {
    PAYPAL,
    STRIPE,
    MAIN
}

pub struct PayoutMethod {
    id: i32,
    user_id: i32,
    user: dyn Future<User>,
    first_name: String,
    last_name: String,
    paypal_email: String,
    birthday: String, // Should be date?
    platform: PlatformType,
    phone: String,
    created_at: String, // Should be date?
    updated_at: String, // Should be date?
}