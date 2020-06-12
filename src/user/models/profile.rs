use tokio_pg_mapper_derive::Postgres_mapper;
use futures::Future;
use super::user::User;
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "profiles")]
pub struct Profile {
    user_id: i32,
    user: dyn Future,
    first_name: String,
    last_name: String,
    age: i32,
    birthday: String, // Should be date?
    bio: String,
    banner: String
}