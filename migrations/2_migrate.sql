CREATE SCHEMA IF NOT EXISTS auth AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS ledger AUTHORIZATION uledger;


-- AUTH --
----------

CREATE TABLE IF NOT EXISTS auth.users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created             TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    role                SMALLINT NOT NULL,

    email               TEXT UNIQUE NOT NULL,
    email_verified_on   TIMESTAMP WITH TIME ZONE,

    password_salt       TEXT NOT NULL,
    password_hash       TEXT NOT NULL,

    display_name        TEXT
);

CREATE TABLE IF NOT EXISTS auth.email_verification (
    user_id         UUID PRIMARY KEY,
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    email_address   TEXT UNIQUE NOT NULL,
    proof_token     TEXT NOT NULL
);

ALTER TABLE auth.email_verification ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);


-- LEDGER --
------------

CREATE TABLE IF NOT EXISTS ledger.accounts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    user_id         UUID NOT NULL,
    kind            SMALLINT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT,
    UNIQUE (user_id, kind, name)
);

CREATE TABLE IF NOT EXISTS ledger.commodities (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    user_id     UUID NOT NULL,
    name        TEXT NOT NULL,
    format      TEXT NOT NULL,
    UNIQUE (user_id, name)
);

CREATE TABLE IF NOT EXISTS ledger.conversions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    user_id         UUID NOT NULL,
    effective       DATE NOT NULL,
    from_commodity  UUID NOT NULL,
    to_commodity    UUID NOT NULL,
    ratio           FLOAT8 NOT NULL,
    UNIQUE (user_id, effective, from_commodity, to_commodity)
);

CREATE TABLE IF NOT EXISTS ledger.payees (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    user_id     UUID NOT NULL,
    name        TEXT NOT NULL,
    UNIQUE (user_id, name)
);

CREATE TABLE IF NOT EXISTS ledger.transactions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    user_id         UUID NOT NULL,
    occurred_on     DATE NOT NULL,
    posted_on       DATE NOT NULL,
    from_account    UUID NOT NULL,
    to_account      UUID NOT NULL,
    change          FLOAT8 NOT NULL,
    from_commodity  UUID NOT NULL,
    to_commodity    UUID NOT NULL,
    payee           UUID NOT NULL,
    description     TEXT
);


ALTER TABLE ledger.accounts ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE ledger.commodities ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE ledger.conversions ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);
ALTER TABLE ledger.conversions ADD FOREIGN KEY (from_commodity) REFERENCES ledger.commodities (id);
ALTER TABLE ledger.conversions ADD FOREIGN KEY (to_commodity) REFERENCES ledger.commodities (id);

ALTER TABLE ledger.payees ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE ledger.transactions ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);
ALTER TABLE ledger.transactions ADD FOREIGN KEY (from_account) REFERENCES ledger.accounts (id);
ALTER TABLE ledger.transactions ADD FOREIGN KEY (to_account) REFERENCES ledger.accounts (id);
ALTER TABLE ledger.transactions ADD FOREIGN KEY (from_commodity) REFERENCES ledger.commodities (id);
ALTER TABLE ledger.transactions ADD FOREIGN KEY (to_commodity) REFERENCES ledger.commodities (id);
ALTER TABLE ledger.transactions ADD FOREIGN KEY (payee) REFERENCES ledger.payees (id);
