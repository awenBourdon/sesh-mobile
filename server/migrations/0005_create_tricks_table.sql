CREATE TABLE IF NOT EXISTS tricks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    spot_id UUID NOT NULL REFERENCES spots(id) ON DELETE CASCADE,
    description TEXT,
    video_url TEXT,
    is_approved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tricks_user_id ON tricks(user_id);
CREATE INDEX IF NOT EXISTS idx_tricks_spot_id ON tricks(spot_id);
CREATE INDEX IF NOT EXISTS idx_tricks_is_approved ON tricks(is_approved);