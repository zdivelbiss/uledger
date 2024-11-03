CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE SCHEMA IF NOT EXISTS _user   AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS _ledger AUTHORIZATION uledger;

CREATE TYPE USER_ACCESS     AS ENUM ('ADMIN', 'REGULAR');
CREATE TYPE ACCOUNT_KIND    AS ENUM ('EQUITY', 'ASSET', 'LIABILITY', 'INCOME', 'EXPENSE');

CREATE DOMAIN AUTO_ID AS UUID
    DEFAULT GEN_RANDOM_UUID();

CREATE DOMAIN CREATETIMEZ AS TIMESTAMP WITH TIME ZONE
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
    id       AUTO_ID PRIMARY KEY,
    created  CREATETIMEZ,

    email_address   EMAIL_ADDRESS   NOT NULL,
    password_salt   TEXT            NOT NULL,
    password_hash   TEXT            NOT NULL,

    pending_email_address         EMAIL_ADDRESS,
    pending_email_address_token   BYTEA,
    pending_email_address_expiry  TIMESTAMP WITH TIME ZONE,

    display_name    TEXT NOT NULL,

    CONSTRAINT profile_unq_email_address
        UNIQUE (email_address),

    CONSTRAINT profile_chk_pending_email_address_token_len
        CHECK (LENGTH(pending_email_address_token) = 3),
    CONSTRAINT profile_chk_display_name_len
        CHECK (CHAR_LENGTH(display_name)  <= 32)
);

CREATE INDEX unq_email_address_1 ON _user.profile(LEAST(email_address, pending_email_address));
CREATE INDEX unq_email_address_2 ON _user.profile(GREATEST(email_address, pending_email_address));

-- LEDGER --
------------

CREATE TABLE IF NOT EXISTS _ledger.account (
    id              AUTO_ID         PRIMARY KEY,
    created         CREATETIMEZ,

    user_id         UUID            NOT NULL,
    kind            ACCOUNT_KIND    NOT NULL,
    name            BTEXT           NOT NULL,
    description     BTEXT,

    CONSTRAINT account_unq
        UNIQUE (user_id, kind, name),

    CONSTRAINT account_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.commodity (
    id           AUTO_ID    PRIMARY KEY,
    created      CREATETIMEZ,

    user_id      UUID       NOT NULL,
    name         BTEXT      NOT NULL,
    format       BTEXT      NOT NULL,

    CONSTRAINT commodity_unq
        UNIQUE (user_id, name),

    CONSTRAINT chk_commodity_format_len
        CHECK (CHAR_LENGTH(format) <= 32),

    CONSTRAINT commodity_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.conversion (
    id              AUTO_ID     PRIMARY KEY,
    created         CREATETIMEZ,

    user_id         UUID        NOT NULL,
    effective       DATE        NOT NULL,
    from_commodity  UUID        NOT NULL,
    to_commodity    UUID        NOT NULL,
    rate            FLOAT8      NOT NULL,

    CONSTRAINT conversion_unq
        UNIQUE (user_id, effective, from_commodity, to_commodity),

    CONSTRAINT commodity_fk_user_id
        FOREIGN KEY (user_id)         REFERENCES _user.profile(id)      ON DELETE CASCADE,
    CONSTRAINT commodity_fk_from_commodity
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.commodity(id)  ON DELETE CASCADE,
    CONSTRAINT commodity_fk_to_commodity
        FOREIGN KEY (to_commodity)    REFERENCES _ledger.commodity(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.payee (
    id          AUTO_ID     PRIMARY KEY,
    created     CREATETIMEZ,

    user_id     UUID        NOT NULL,
    name        BTEXT       NOT NULL,

    CONSTRAINT payee_unq
        UNIQUE (user_id, name),

    CONSTRAINT payee_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _ledger.transaction (
    id              AUTO_ID     PRIMARY KEY,
    created         CREATETIMEZ,

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
        FOREIGN KEY (from_commodity)  REFERENCES _ledger.payee(id)      ON DELETE CASCADE
);