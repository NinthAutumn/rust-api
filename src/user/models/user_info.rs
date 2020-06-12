use tokio_pg_mapper_derive::PostgresMapper;
use super::user::User;
#[derive(PostgresMapper)]
#[pg_mapper(table = "user_infos")]
pub struct UserInfo {
    user_id: i32,
    user: User,
    login_count: i32,
    last_logged_in: String, // Should be date?
    created_ip: String,
    last_logged_in_ip: String,
    country_code: String,
    continent_code: String,
    phone_code: String,
    timezone: String,
    updated_at: String, // Should be date?
}