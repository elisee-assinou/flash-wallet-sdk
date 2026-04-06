CREATE TABLE balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    momo_number VARCHAR NOT NULL UNIQUE,
    balance_sats BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
