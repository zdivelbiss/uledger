CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE SCHEMA IF NOT EXISTS _user   AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS _ledger AUTHORIZATION uledger;

CREATE TYPE USER_ACCESS     AS ENUM ('ADMIN', 'REGULAR');
CREATE TYPE ACCOUNT_KIND    AS ENUM ('EQUITY', 'ASSET', 'LIABILITY', 'INCOME', 'EXPENSE');

CREATE DOMAIN EMAIL_ADDRESS AS CITEXT
    CHECK (CHAR_LENGTH((value)) <= 128)
    CHECK ((value) ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$')
;

-- AUTH --
----------

CREATE TABLE IF NOT EXISTS _user.profile (
    id                  UUID            PRIMARY KEY     DEFAULT GEN_RANDOM_UUID(),
    created             TIMESTAMP       NOT NULL        DEFAULT NOW(),

    email_address       EMAIL_ADDRESS   NOT NULL        UNIQUE,
    email_verified_on   TIMESTAMP WITH TIME ZONE,
    password_salt       TEXT            NOT NULL,
    password_hash       TEXT            NOT NULL,

    access              USER_ACCESS     NOT NULL,
    display_name        TEXT            NOT NULL,

    CHECK               (CHAR_LENGTH(display_name)  <= 32)
);

CREATE TABLE IF NOT EXISTS _user.email_verification (
    id              UUID            PRIMARY KEY,
    created         TIMESTAMP       NOT NULL    DEFAULT NOW(),

    email_address   EMAIL_ADDRESS   NOT NULL    UNIQUE,
    proof_token     TEXT            NOT NULL,

    FOREIGN KEY     (id) REFERENCES _user.profile(id) ON DELETE CASCADE
);


-- LEDGER --
------------

CREATE TABLE IF NOT EXISTS _ledger.account (
    id              UUID           PRIMARY KEY  DEFAULT GEN_RANDOM_UUID(),
    created         TIMESTAMP      NOT NULL     DEFAULT NOW(),

    user_id         UUID           NOT NULL,
    kind            ACCOUNT_KIND   NOT NULL,
    name            CITEXT         NOT NULL,
    description     TEXT,

    UNIQUE          (user_id, kind, name),
    FOREIGN KEY     (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.commodity (
    id           UUID       PRIMARY KEY  DEFAULT GEN_RANDOM_UUID(),
    created      TIMESTAMP  NOT NULL     DEFAULT NOW(),

    user_id      UUID       NOT NULL,
    name         CITEXT     NOT NULL,
    format       TEXT       NOT NULL,

    UNIQUE       (user_id, name),
    FOREIGN KEY  (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.conversion (
    id              UUID        PRIMARY KEY  DEFAULT GEN_RANDOM_UUID(),
    created         TIMESTAMP   NOT NULL     DEFAULT NOW(),

    user_id         UUID        NOT NULL,
    effective       DATE        NOT NULL,
    from_commodity  UUID        NOT NULL,
    to_commodity    UUID        NOT NULL,
    rate            FLOAT8      NOT NULL,

    UNIQUE          (user_id, effective, from_commodity, to_commodity),
    FOREIGN KEY     (user_id)         REFERENCES _user.profile(id)      ON DELETE CASCADE,
    FOREIGN KEY     (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (to_commodity)    REFERENCES _ledger.commodity(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.payee (
    id          UUID        PRIMARY KEY  DEFAULT GEN_RANDOM_UUID(),
    created     TIMESTAMP   NOT NULL     DEFAULT NOW(),

    user_id     UUID NOT NULL,
    name        CITEXT NOT NULL,

    UNIQUE      (user_id, name),
    FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.transaction (
    id              UUID        PRIMARY KEY  DEFAULT GEN_RANDOM_UUID(),
    created         TIMESTAMP   NOT NULL     DEFAULT NOW(),

    user_id         UUID        NOT NULL,

    occurred_on     DATE        NOT NULL,
    posted_on       DATE        NOT NULL,
    from_account    UUID        NOT NULL,
    to_account      UUID        NOT NULL,
    change          FLOAT8      NOT NULL,
    from_commodity  UUID        NOT NULL,
    to_commodity    UUID        NOT NULL,
    payee           UUID        NOT NULL,
    description     TEXT,

    FOREIGN KEY     (user_id)         REFERENCES _user.profile(id)      ON DELETE CASCADE,
    FOREIGN KEY     (from_account)    REFERENCES _ledger.account(id)    ON DELETE CASCADE,
    FOREIGN KEY     (to_account)      REFERENCES _ledger.account(id)    ON DELETE CASCADE,
    FOREIGN KEY     (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (to_commodity)    REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    FOREIGN KEY     (payee)           REFERENCES _ledger.payee(id)      ON DELETE CASCADE
);