CREATE TABLE wallets (
    id SERIAL PRIMARY KEY,

    public_address VARCHAR(255) UNIQUE NOT NULL, 

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

 