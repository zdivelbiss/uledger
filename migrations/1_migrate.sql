CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE SCHEMA IF NOT EXISTS users  AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS ledger AUTHORIZATION uledger;

CREATE TYPE USER_ACCESS  AS ENUM ('ADMIN', 'REGULAR');
CREATE TYPE ACCOUNT_KIND AS ENUM ('EQUITY', 'ASSET', 'LIABILITY', 'INCOME', 'EXPENSE');

-- AUTH --
----------

CREATE TABLE IF NOT EXISTS users.account (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created             TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    access              USER_ACCESS NOT NULL,

    email_address       CITEXT UNIQUE NOT NULL,
    email_verified_on   TIMESTAMP WITH TIME ZONE,

    password_salt       TEXT NOT NULL,
    password_hash       TEXT NOT NULL,

    display_name        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users.profile (
    id                  UUID PRIMARY KEY,

    display_name        TEXT NOT NULL,

    FOREIGN KEY         (id) REFERENCES users.account(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users.email_verification (
    id              UUID PRIMARY KEY,
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    email_address   CITEXT UNIQUE NOT NULL,
    proof_token     TEXT NOT NULL,

    FOREIGN KEY     (id) REFERENCES users.account(id) ON DELETE CASCADE
);


-- LEDGER --
------------

CREATE TABLE IF NOT EXISTS ledger.account (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    kind            ACCOUNT_KIND NOT NULL,
    name            CITEXT NOT NULL,
    description     TEXT,

    UNIQUE          (user_id, kind, name),
    FOREIGN KEY     (user_id) REFERENCES users.account(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ledger.commodity (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL,
    created      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    name         CITEXT NOT NULL,
    format       TEXT NOT NULL,

    UNIQUE       (user_id, name),
    FOREIGN KEY  (user_id) REFERENCES users.account(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ledger.conversion (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    effective       DATE NOT NULL,
    from_commodity  UUID NOT NULL,
    to_commodity    UUID NOT NULL,
    rate            FLOAT8 NOT NULL,

    UNIQUE          (user_id, effective, from_commodity, to_commodity),
    FOREIGN KEY     (user_id)         REFERENCES users.account(id)     ON DELETE CASCADE,
    FOREIGN KEY     (from_commodity)  REFERENCES ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (to_commodity)    REFERENCES ledger.commodity(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ledger.payee (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL,
    created     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    name        CITEXT NOT NULL,

    UNIQUE      (user_id, name),
    FOREIGN KEY (user_id) REFERENCES users.account(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ledger.transaction (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    occurred_on     DATE NOT NULL,
    posted_on       DATE NOT NULL,
    from_account    UUID NOT NULL,
    to_account      UUID NOT NULL,
    change          FLOAT8 NOT NULL,
    from_commodity  UUID NOT NULL,
    to_commodity    UUID NOT NULL,
    payee           UUID NOT NULL,
    description     TEXT,

    FOREIGN KEY     (user_id)         REFERENCES users.account(id)     ON DELETE CASCADE,
    FOREIGN KEY     (from_account)    REFERENCES ledger.account(id)    ON DELETE CASCADE,
    FOREIGN KEY     (to_account)      REFERENCES ledger.account(id)    ON DELETE CASCADE,
    FOREIGN KEY     (from_commodity)  REFERENCES ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (to_commodity)    REFERENCES ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (payee)           REFERENCES ledger.payee(id)      ON DELETE CASCADE
);