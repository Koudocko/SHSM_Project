CREATE TABLE users (
    id numeric NOT NULL,
    username character varying(100) NOT NULL,
    password character varying(100) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE TABLE test (
    id numeric NOT NULL,
    CONSTRAINT abc PRIMARY KEY (id)
);

CREATE TABLE j (
    id numeric NOT NULL,
    CONSTRAINT abcd PRIMARY KEY (id)
);