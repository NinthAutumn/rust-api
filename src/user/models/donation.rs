use tokio_pg_mapper_derive::PostgresMapper;
use super::user::User;
#[derive(PostgresMapper)]
#[pg_mapper(table = "user_donation_links")]
pub struct UserDonationLinks {
    user_id: i32,
    user: User,
    donation: Vec<String>,
    updated_at: String, // Should be date?
}
