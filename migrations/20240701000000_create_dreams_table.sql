-- Create the extensions if they don't exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create dreams table
CREATE TABLE IF NOT EXISTS dreams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    interpretation TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
); 