use tokio_pg_mapper_derive::Postgres_mapper;
use futures::Future;
use super::user::User;
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "user_donation_links")]
pub struct UserDonationLinks {
    user_id: i32,
    user: User,
    donation: Vec<String>
}
