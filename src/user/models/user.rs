use super::{setting, wallet};
use futures::Future;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{FromSql, Timestamp, ToSql};

#[derive(Debug, ToSql, FromSql)]
pub enum Gender {
    MALE,
    FEMALE,
    OTHER,
}

#[derive(Debug, ToSql, FromSql)]
pub enum Strategies {
    FACEBOOK,
    GOOGLE,
    LOCAL,
    TWITTER,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
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
    created_at: Timestamp, // Should be date?
    updated_at: Timestamp, // Should be date?
}
