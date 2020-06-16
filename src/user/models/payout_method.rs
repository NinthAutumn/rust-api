use super::user::User;
use futures::Future;
use tokio_pg_mapper_derive::PostgresMapper;

pub enum PlatformType {
    PAYPAL,
    STRIPE,
    MAIN,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "payout_methods")]
pub struct PayoutMethod {
    id: i32,
    user_id: i32,
    first_name: String,
    last_name: String,
    paypal_email: String,
    birthday: String, // Should be date?
    platform: PlatformType,
    phone: String,
    created_at: String, // Should be date?
    updated_at: String, // Should be date?
}
