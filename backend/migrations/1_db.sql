CREATE SCHEMA IF NOT EXISTS public AUTHORIZATION uledger;
CREATE SCHEMA IF NOT EXISTS auth AUTHORIZATION uledger;

-- AUTH --
----------

CREATE TABLE IF NOT EXISTS auth.users (
    id                  UUID PRIMARY KEY,
    created             TIMESTAMP WITH TIME ZONE NOT NULL,
    role                TEXT NOT NULL,
    email               TEXT NOT NULL,
    email_confirmed_on  DATE,
    password            TEXT NOT NULL
);



-- PUBLIC --
------------

CREATE TABLE IF NOT EXISTS public.accounts (
    id              UUID PRIMARY KEY,
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL,
    kind            TEXT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT
);

CREATE TABLE IF NOT EXISTS public.commodities (
    id          UUID PRIMARY KEY,
    user_id     UUID NOT NULL,
    created     TIMESTAMP WITH TIME ZONE NOT NULL,
    name        TEXT NOT NULL,
    format      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS public.conversions (
    id              UUID PRIMARY KEY,
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL,
    effective       DATE NOT NULL,
    from_commodity  UUID NOT NULL,
    to_commodity    UUID NOT NULL,
    ratio           FLOAT8 NOT NULL
);

CREATE TABLE IF NOT EXISTS public.payees (
    id          UUID PRIMARY KEY,
    user_id     UUID NOT NULL,
    created     TIMESTAMP WITH TIME ZONE NOT NULL,
    name        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS public.transactions (
    id              UUID PRIMARY KEY,
    user_id         UUID NOT NULL,
    created         TIMESTAMP WITH TIME ZONE NOT NULL,
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


ALTER TABLE public.accounts ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE public.commodities ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE public.conversions ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);
ALTER TABLE public.conversions ADD FOREIGN KEY (from_commodity) REFERENCES public.commodities (id);
ALTER TABLE public.conversions ADD FOREIGN KEY (to_commodity) REFERENCES public.commodities (id);

ALTER TABLE public.payees ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);

ALTER TABLE public.transactions ADD FOREIGN KEY (user_id) REFERENCES auth.users (id);
ALTER TABLE public.transactions ADD FOREIGN KEY (from_account) REFERENCES public.accounts (id);
ALTER TABLE public.transactions ADD FOREIGN KEY (to_account) REFERENCES public.accounts (id);
ALTER TABLE public.transactions ADD FOREIGN KEY (from_commodity) REFERENCES public.commodities (id);
ALTER TABLE public.transactions ADD FOREIGN KEY (to_commodity) REFERENCES public.commodities (id);
ALTER TABLE public.transactions ADD FOREIGN KEY (payee) REFERENCES public.payees (id);
