#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub guild_id: i64,
    pub channel_id: i64,
    pub enabled: bool,
    pub hook_id: i64,
    pub hook_token: String,
    pub tag: String,
}

impl Hook {
    pub async fn get_by_guild_id(
        pool: &sqlx::PgPool,
        guild_id: i64,
    ) -> Result<Vec<Hook>, sqlx::Error> {
        sqlx::query_as!(Hook, "SELECT * FROM hooks WHERE guild_id = $1", guild_id)
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_guild_and_channel(
        pool: &sqlx::PgPool,
        guild_id: i64,
        channel_id: i64,
    ) -> Result<Vec<Hook>, sqlx::Error> {
        sqlx::query_as!(
            Hook,
            "SELECT * FROM hooks WHERE guild_id = $1 AND channel_id = $2",
            guild_id,
            channel_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<Hook>, sqlx::Error> {
        sqlx::query_as!(Hook, "SELECT * FROM hooks")
            .fetch_all(pool)
            .await
    }

    pub async fn insert(
        pool: &sqlx::PgPool,
        guild_id: i64,
        channel_id: i64,
        hook_id: i64,
        hook_token: String,
        tag: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO hooks (guild_id, channel_id, enabled, hook_id, hook_token, tag) VALUES ($1, $2, $3, $4, $5, $6)",
            guild_id,
            channel_id,
            true,
            hook_id,
            hook_token,
            tag
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_enabled(&self, pool: &sqlx::PgPool, enabled: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE hooks SET enabled = $1 WHERE guild_id = $2 AND channel_id = $3",
            enabled,
            self.guild_id,
            self.channel_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_tag(&self, pool: &sqlx::PgPool, tag: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE hooks SET tag = $1 WHERE guild_id = $2 AND channel_id = $3",
            tag,
            self.guild_id,
            self.channel_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
