use tokio_pg_mapper_derive::PostgresMapper;
use futures::Future;
use super::{wallet, setting};


pub enum Gender{
    MALE,
    FEMALE,
    OTHER
}

pub enum Strategies {
    FACEBOOK,
    GOOGLE,
    LOCAL,
    TWITTER
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User{
    id: i32,
    avatar: String,
    gender: Gender,
    email: String,
    verified: bool,
    stripe_customer_id: String,
    password: String,
    refresh_token: String,
    social_id: String,
    strategy: Strategies,
    wallet: dyn Future<wallet::Wallet>,
    roles: dyn Future,
    setting: dyn Future<setting::Setting>,
    created_at: String, // Should be date?
    updated_at: String, // Should be date?
}