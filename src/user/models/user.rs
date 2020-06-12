use tokio_pg_mapper_derive::Postgres_mapper;
use futures::Future;


enum Gender{
    MALE,
    FEMALE,
    OTHER
}

enum Strategies {
    FACEBOOK,
    GOOGLE,
    LOCAL,
    TWITTER
}

#[derive(Serialize, Deserialize, PostgresMapper)]
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
    wallet: dyn Future,
    roles: dyn Future,
    setting: dyn Future
}