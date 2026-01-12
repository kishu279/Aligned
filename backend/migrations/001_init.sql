-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR(255),
    is_profile_complete BOOLEAN DEFAULT FALSE,
    is_new_user BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_active TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    preferences JSONB DEFAULT '{}'
);

-- 2. Profiles Table
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100),
    bio TEXT,
    birthdate DATE,
    pronouns VARCHAR(50),
    gender VARCHAR(50),
    sexuality VARCHAR(50),
    height INTEGER, -- in cm
    location POINT,
    job VARCHAR(100),
    company VARCHAR(100),
    school VARCHAR(100),
    ethnicity VARCHAR(50),
    politics VARCHAR(50),
    religion VARCHAR(50),
    relationship_type VARCHAR(100),
    dating_intention VARCHAR(100),
    drinks VARCHAR(50),
    smokes VARCHAR(50),
    image_count INTEGER DEFAULT 0
);

-- 3. User Images Table
CREATE TABLE user_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    display_order INTEGER NOT NULL, -- 0 to 5
    caption TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 4. User Prompts Table
CREATE TABLE user_prompts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    display_order INTEGER NOT NULL, -- 0 to 2
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 5. Interactions / Likes Table
CREATE TABLE interactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    from_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    to_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL, -- 'LIKE', 'PASS'
    context_type VARCHAR(20), -- 'IMAGE', 'PROMPT'
    context_id VARCHAR(255),
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(from_user_id, to_user_id) -- Prevent duplicate interactions
);

-- 6. Matches Table
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user1_id UUID REFERENCES users(id) ON DELETE CASCADE,
    user2_id UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_message TEXT,
    last_message_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(user1_id, user2_id)
);

-- 7. Messages Table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    match_id UUID REFERENCES matches(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_read BOOLEAN DEFAULT FALSE
);

-- Indexes for performance
CREATE INDEX idx_profiles_location ON profiles USING GIST(location);
CREATE INDEX idx_interactions_from_user ON interactions(from_user_id);
CREATE INDEX idx_interactions_to_user ON interactions(to_user_id);
CREATE INDEX idx_matches_user1 ON matches(user1_id);
CREATE INDEX idx_matches_user2 ON matches(user2_id);
CREATE INDEX idx_messages_match_id ON messages(match_id);
CREATE INDEX idx_messages_created_at ON messages(created_at DESC);
