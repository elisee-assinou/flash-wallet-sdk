CREATE TABLE wallet_config (
    id UUID PRIMARY KEY,
    lightning_address VARCHAR(255) NOT NULL,
    momo_number VARCHAR(20) NOT NULL,
    convert_ratio FLOAT NOT NULL DEFAULT 1.0,
    is_auto_convert BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
