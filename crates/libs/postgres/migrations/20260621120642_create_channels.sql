CREATE TYPE channel_type AS ENUM (
    'direct',
    'guild_text'
);

CREATE TABLE IF NOT EXISTS public.channels (
    id CHAR(26) PRIMARY KEY,

    channel_type channel_type NOT NULL,
    guild_id CHAR(26) DEFAULT NULL REFERENCES public.guilds(id) ON DELETE CASCADE,
    name TEXT DEFAULT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT channel_name_length CHECK (
        name IS NULL
        OR char_length(name) BETWEEN 1 AND 100
    ),
    CONSTRAINT guild_text_requires_guild CHECK (
        channel_type <> 'guild_text'
        OR guild_id IS NOT NULL
    ),
    CONSTRAINT guild_text_requires_name CHECK (
        channel_type <> 'guild_text'
        OR name IS NOT NULL
    )
);

CREATE TABLE IF NOT EXISTS public.channel_members (
    channel_id CHAR(26) NOT NULL REFERENCES public.channels(id) ON DELETE CASCADE,
    user_id CHAR(26) NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,

    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (channel_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_channels_guild_id
    ON channels (guild_id);

CREATE INDEX IF NOT EXISTS idx_channel_members_user_id
    ON channel_members (user_id);
