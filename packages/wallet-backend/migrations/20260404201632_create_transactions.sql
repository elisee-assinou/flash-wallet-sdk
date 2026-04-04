CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    transaction_type VARCHAR(20) NOT NULL,
    amount_xof BIGINT NOT NULL,
    amount_sats BIGINT NOT NULL,
    exchange_rate BIGINT NOT NULL,
    momo_number VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
    payment_hash VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
