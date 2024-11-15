-- META --
----------

CREATE EXTENSION "citext";
CREATE EXTENSION "uuid-ossp";
CREATE EXTENSION "pgcrypto";

CREATE SCHEMA _user   AUTHORIZATION uledger;
CREATE SCHEMA _ledger AUTHORIZATION uledger;

CREATE TYPE USER_ACCESS     AS ENUM ('ADMIN', 'REGULAR');

CREATE DOMAIN AUTO_ID AS UUID
    DEFAULT GEN_RANDOM_UUID();

CREATE DOMAIN TIMESTAMPZ AS TIMESTAMP WITH TIME ZONE
    DEFAULT NOW();

CREATE DOMAIN EMAIL_ADDRESS AS CITEXT
    CONSTRAINT chk_email_address_len
        CHECK (CHAR_LENGTH((value)) <= 128)
    CONSTRAINT chk_email_address_format
        CHECK ((value) ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$');



-- AUTH --
----------

CREATE TABLE _user.profile (
    id       AUTO_ID PRIMARY KEY,
    created  TIMESTAMPZ NOT NULL,

    email_address   EMAIL_ADDRESS   NOT NULL,
    password_salt   TEXT            NOT NULL,
    password_hash   TEXT            NOT NULL,

    pending_email_address         EMAIL_ADDRESS,
    pending_email_address_token   BYTEA,
    pending_email_address_expiry  TIMESTAMP WITH TIME ZONE,

    display_name    TEXT NOT NULL,

    CONSTRAINT profile_chk_pending_email_address_token_len
        CHECK (LENGTH(pending_email_address_token) = 3),
    CONSTRAINT profile_chk_display_name_len
        CHECK (CHAR_LENGTH(display_name)  <= 32)
);

CREATE UNIQUE INDEX profile_unq_1 ON _user.profile(LEAST(email_address, pending_email_address));
CREATE UNIQUE INDEX profile_unq_2 ON _user.profile(GREATEST(email_address, pending_email_address));



-- LEDGER --
------------

CREATE TABLE _ledger.account (
    id              AUTO_ID     PRIMARY KEY,
    created         TIMESTAMPZ  NOT NULL,
    user_id         UUID        NOT NULL,

    name            TEXT NOT NULL,
    description     TEXT,

    CONSTRAINT account_unq
        UNIQUE (user_id, kind, name),

    CONSTRAINT account_chk_name_len
        CHECK (CHAR_LENGTH(name) <= 128),
    CONSTRAINT account_chk_description_len
        CHECK (CHAR_LENGTH(description) <= 1024),

    CONSTRAINT account_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE _ledger.commodity (
    id           AUTO_ID     PRIMARY KEY,
    created      TIMESTAMPZ  NOT NULL,
    user_id      UUID        NOT NULL,

    name                 TEXT     NOT NULL,
    description          TEXT,
    symbol               TEXT     NOT NULL,
    thousands_separator  TEXT     NOT NULL,
    decimal_separator    TEXT     NOT NULL,
    is_prefix            BOOLEAN  NOT NULL,

    CONSTRAINT commodity_unq
        UNIQUE (user_id, name),

    CONSTRAINT chk_commodity_name_len
        CHECK (CHAR_LENGTH(name) <= 128),
    CONSTRAINT chk_commodity_description_len
        CHECK (CHAR_LENGTH(description) <= 1024),
    CONSTRAINT chk_commodity_symbol_len
        CHECK (CHAR_LENGTH(symbol) <= 16),
    CONSTRAINT chk_commodity_thousands_separator_len
        CHECK (CHAR_LENGTH(thousands_separator) <= 1),
    CONSTRAINT chk_commodity_decimal_separator_len
        CHECK (CHAR_LENGTH(decimal_separator) <= 1),

    CONSTRAINT commodity_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE _ledger.conversion (
    id              AUTO_ID     PRIMARY KEY,
    created         TIMESTAMPZ  NOT NULL,
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

CREATE TABLE _ledger.payee (
    id       AUTO_ID     PRIMARY KEY,
    created  TIMESTAMPZ  NOT NULL,
    user_id  UUID        NOT NULL,

    name         TEXT NOT NULL,
    description  TEXT,

    CONSTRAINT payee_unq
        UNIQUE (user_id, name),

    CONSTRAINT payee_chk_name_len
        CHECK (CHAR_LENGTH(name) <= 128),
    CONSTRAINT payee_chk_description_len
        CHECK (CHAR_LENGTH(description) <= 1024),

    CONSTRAINT payee_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _user.profile(id) ON DELETE CASCADE
);

CREATE TABLE _ledger.transaction (
    id              AUTO_ID     PRIMARY KEY,
    created         TIMESTAMPZ  NOT NULL,
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

    CONSTRAINT transaction_chk_description_len
        CHECK (CHAR_LENGTH(description) <= 1024),

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



-- TAGS --
----------

CREATE TABLE _ledger.account_tag (
    id          AUTO_ID     PRIMARY KEY,
    created     TIMESTAMPZ  NOT NULL,
    user_id     UUID        NOT NULL,

    name  TEXT NOT NULL,

    CONSTRAINT account_tag_unq
        UNIQUE(user_id, name),

    CONSTRAINT account_tag_chk_name_len
        CHECK (CHAR_LENGTH(name) <= 32),

    CONSTRAINT account_tag_fk_user_id
        FOREIGN KEY (user_id) REFERENCES _users.profile(id) ON DELETE CASCADE
);

CREATE TABLE _ledger.account_tag_map (
    id          AUTO_ID PRIMARY KEY,
    account_id  UUID    NOT NULL,
    tag_id      UUID    NOT NULL,

    CONSTRAINT account_tag_map_fk_account_id
        FOREIGN KEY (account_id)  REFERENCES _ledger.account(id)      ON DELETE CASCADE,
    CONSTRAINT account_tag_map_fk_tag_id
        FOREIGN KEY (tag_id)      REFERENCES _ledger.account_tag(id)  ON DELETE CASCADE
);