-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create transactions table
CREATE TABLE transactions (
    signature VARCHAR(88) PRIMARY KEY,
    slot BIGINT NOT NULL,
    block_time TIMESTAMPTZ NOT NULL,
    fee BIGINT NOT NULL,
    status VARCHAR(50) NOT NULL,
    instructions_json JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create token_accounts table
CREATE TABLE token_accounts (
    pubkey VARCHAR(44) PRIMARY KEY,
    mint VARCHAR(44) NOT NULL,
    owner VARCHAR(44) NOT NULL,
    amount BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create price_history table
CREATE TABLE price_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token_mint VARCHAR(44) NOT NULL,
    price_usd DECIMAL(20, 8) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    source VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create protocol_interactions table
CREATE TABLE protocol_interactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet VARCHAR(44) NOT NULL,
    protocol VARCHAR(100) NOT NULL,
    interaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create governance_votes table
CREATE TABLE governance_votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    voter VARCHAR(44) NOT NULL,
    proposal_id VARCHAR(100) NOT NULL,
    vote VARCHAR(50) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    dao_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for time-based queries
CREATE INDEX idx_transactions_block_time ON transactions(block_time);
CREATE INDEX idx_token_accounts_updated_at ON token_accounts(updated_at);
CREATE INDEX idx_price_history_timestamp ON price_history(timestamp);
CREATE INDEX idx_protocol_interactions_timestamp ON protocol_interactions(timestamp);
CREATE INDEX idx_governance_votes_timestamp ON governance_votes(timestamp);

-- Create indexes for common query patterns
CREATE INDEX idx_token_accounts_mint ON token_accounts(mint);
CREATE INDEX idx_token_accounts_owner ON token_accounts(owner);
CREATE INDEX idx_price_history_token_mint ON price_history(token_mint);
CREATE INDEX idx_protocol_interactions_wallet ON protocol_interactions(wallet);
CREATE INDEX idx_protocol_interactions_protocol ON protocol_interactions(protocol);
CREATE INDEX idx_governance_votes_voter ON governance_votes(voter);
CREATE INDEX idx_governance_votes_proposal ON governance_votes(proposal_id);

-- Add foreign key constraints where applicable
ALTER TABLE token_accounts
    ADD CONSTRAINT fk_token_accounts_mint
    FOREIGN KEY (mint)
    REFERENCES token_accounts(pubkey)
    ON DELETE CASCADE;

-- Add check constraints
ALTER TABLE price_history
    ADD CONSTRAINT check_price_history_price_usd
    CHECK (price_usd >= 0);

ALTER TABLE protocol_interactions
    ADD CONSTRAINT check_protocol_interactions_amount
    CHECK (amount >= 0); 