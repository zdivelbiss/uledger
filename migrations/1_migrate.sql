CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE SCHEMA IF NOT EXISTS _user   AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS _ledger AUTHORIZATION uledger;

CREATE TYPE USER_ACCESS     AS ENUM ('ADMIN', 'REGULAR');
CREATE TYPE ACCOUNT_KIND    AS ENUM ('EQUITY', 'ASSET', 'LIABILITY', 'INCOME', 'EXPENSE');

CREATE DOMAIN AUTO_ID AS UUID
    DEFAULT GEN_RANDOM_UUID();

CREATE DOMAIN TIMESTAMPZ AS TIMESTAMP WITH TIME ZONE
    DEFAULT NOW()
    NOT NULL;

CREATE DOMAIN EMAIL_ADDRESS AS CITEXT
    CONSTRAINT chk_email_address_len
        CHECK (CHAR_LENGTH((value)) <= 128)
    CONSTRAINT chk_email_address_format
        CHECK ((value) ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$');

CREATE DOMAIN BTEXT AS TEXT
    CONSTRAINT chk_btext_len
        CHECK(CHAR_LENGTH((value)) <= 256);

-- AUTH --
----------

CREATE TABLE IF NOT EXISTS _user.profile (
    id                  AUTO_ID         PRIMARY KEY,
    created             TIMESTAMPZ,

    email_address       EMAIL_ADDRESS   NOT NULL,
    email_verified_on   TIMESTAMP WITH TIME ZONE,
    password_salt       BTEXT           NOT NULL,
    password_hash       BTEXT           NOT NULL,

    display_name        BTEXT           NOT NULL,

    CONSTRAINT profile_unique_email_address
        UNIQUE (email_address),

    CONSTRAINT profile_chk_display_name_len
        CHECK (CHAR_LENGTH(display_name)  <= 32),
);

CREATE TABLE IF NOT EXISTS _user.verification (
    id              AUTO_ID         PRIMARY KEY,
    created         TIMESTAMPZ,

    email_address   EMAIL_ADDRESS   NOT NULL    UNIQUE,
    proof_token     TEXT            NOT NULL,

    CONSTRAINT verification_unique_email_address
        UNIQUE (email_address),

    CONSTRAINT email_verification_chk_proof_token_len
        CHECK (CHAR_LENGTH(proof_token) = 6),

    CONSTRAINT verification_fk_user_id
        FOREIGN KEY (id) REFERENCES _user.profile(id) ON DELETE CASCADE,
);


-- LEDGER --
------------

CREATE TABLE IF NOT EXISTS _ledger.account (
    id              AUTO_ID         PRIMARY KEY,
    created         TIMESTAMPZ,

    user_id         UUID            NOT NULL,
    kind            ACCOUNT_KIND    NOT NULL,
    name            BTEXT           NOT NULL,
    description     BTEXT,

    CONSTRAINT account_unique
        UNIQUE (user_id, kind, name),

    CONSTRAINT account_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS _ledger.commodity (
    id           AUTO_ID    PRIMARY KEY,
    created      TIMESTAMPZ,

    user_id      UUID       NOT NULL,
    name         BTEXT      NOT NULL,
    format       BTEXT      NOT NULL,

    CONSTRAINT commodity_unique
        UNIQUE (user_id, name),

    CONSTRAINT chk_commodity_format_len
        CHECK (CHAR_LENGTH(format) <= 32),

    CONSTRAINT commodity_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS _ledger.conversion (
    id              AUTO_ID     PRIMARY KEY,
    created         TIMESTAMPZ,

    user_id         UUID        NOT NULL,
    effective       DATE        NOT NULL,
    from_commodity  UUID        NOT NULL,
    to_commodity    UUID        NOT NULL,
    rate            FLOAT8      NOT NULL,

    CONSTRAINT conversion_unique
        UNIQUE (user_id, effective, from_commodity, to_commodity),

    CONSTRAINT commodity_fk_user_id
        FOREIGN KEY (user_id)         REFERENCES _user.profile(id)      ON DELETE CASCADE,
    CONSTRAINT commodity_fk_from_commodity
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    CONSTRAINT commodity_fk_to_commodity
        FOREIGN KEY (to_commodity)    REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS _ledger.payee (
    id          AUTO_ID     PRIMARY KEY,
    created     TIMESTAMPZ,

    user_id     UUID        NOT NULL,
    name        BTEXT       NOT NULL,

    CONSTRAINT payee_unique
        UNIQUE (user_id, name),

    CONSTRAINT payee_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS _ledger.transaction (
    id              AUTO_ID     PRIMARY KEY,
    created         TIMESTAMPZ,

    user_id         UUID        NOT NULL,
    occurred_on     DATE        NOT NULL,
    posted_on       DATE        NOT NULL,
    from_account    UUID        NOT NULL,
    to_account      UUID        NOT NULL,
    change          FLOAT8      NOT NULL,
    from_commodity  UUID        NOT NULL,
    to_commodity    UUID        NOT NULL,
    payee           UUID        NOT NULL,
    description     BTEXT,

    CONSTRAINT transaction_fk_user_id
        FOREIGN KEY (user_id)         REFERENCES _user.profile(id)      ON DELETE CASCADE,
    CONSTRAINT transaction_fk_from_account
        FOREIGN KEY (from_account)    REFERENCES _ledger.account(id)    ON DELETE CASCADE,
    CONSTRAINT transaction_fk_to_account
        FOREIGN KEY (to_account)      REFERENCES _ledger.account(id)    ON DELETE CASCADE,
    CONSTRAINT transaction_fk_from_commodity
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    CONSTRAINT transaction_fk_to_commodity
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    CONSTRAINT transaction_fk_payee
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.payee(id)      ON DELETE CASCADE,
);