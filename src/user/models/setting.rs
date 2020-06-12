use tokio_pg_mapper_derive::Postgres_mapper;
use super::user::User;
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "settings")]
pub struct Setting {
    user_id: i32,
    user: User,
    chapter_font_size: i32,
    chapter_font_family: String,
    main_theme: String,
    ranking_display: String,
    update_display: String
}