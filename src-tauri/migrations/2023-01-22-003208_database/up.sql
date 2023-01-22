CREATE TABLE users (
    id numeric NOT NULL,
    username character varying(100) NOT NULL,
    password character varying(100) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
)