-- Add migration script here
CREATE TABLE IF NOT EXISTS hooks (
    guild_id bigint NOT NULL,
    channel_id bigint NOT NULL,
    enabled boolean NOT NULL,
    hook_id bigint NOT NULL,
    hook_token text NOT NULL,
    tag text NOT NULL
);
