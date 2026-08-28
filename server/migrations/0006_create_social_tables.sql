CREATE TABLE IF NOT EXISTS trick_likes (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trick_id UUID NOT NULL REFERENCES tricks(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, trick_id)
);

CREATE TABLE IF NOT EXISTS trick_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trick_id UUID NOT NULL REFERENCES tricks(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_trick_likes_trick_id ON trick_likes(trick_id);
CREATE INDEX IF NOT EXISTS idx_trick_comments_trick_id ON trick_comments(trick_id);
