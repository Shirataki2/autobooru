#[derive(Clone)]
pub struct Data {
    pub pool: sqlx::PgPool,
    pub client: crate::client::BooruClient,
}
