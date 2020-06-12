use tokio_pg_mapper_derive::PostgresMapper;
use futures::Future;
use super::user::User;
#[derive(PostgresMapper)]
#[pg_mapper(table = "profiles")]
pub struct Profile {
    user_id: i32,
    user: dyn Future<User>,
    first_name: String,
    last_name: String,
    age: i32,
    birthday: String, // Should be date?
    bio: String,
    banner: String,
    updated_at: String, // Should be date?
}