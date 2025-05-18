-- Create dream_tags table
CREATE TABLE IF NOT EXISTS dream_tags (
    dream_id UUID NOT NULL REFERENCES dreams(id) ON DELETE CASCADE,
    tag TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    PRIMARY KEY (dream_id, tag)
);

-- Create index for faster tag lookups
CREATE INDEX IF NOT EXISTS idx_dream_tags_tag ON dream_tags(tag);

-- Add search index to dreams table for full-text search
ALTER TABLE dreams ADD COLUMN IF NOT EXISTS search_vector tsvector
    GENERATED ALWAYS AS (to_tsvector('english', text || ' ' || interpretation)) STORED;

CREATE INDEX IF NOT EXISTS idx_dreams_search ON dreams USING GIN(search_vector); 