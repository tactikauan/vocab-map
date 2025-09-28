CREATE TABLE embeddings (
    word character varying NOT NULL PRIMARY KEY,
    vector vector(300) NOT NULL
);
