CREATE TABLE vocab (
	id serial4 NOT NULL PRIMARY KEY,
	word character varying NOT NULL,
    lang Language NOT NULL
);

CREATE UNIQUE INDEX vocab_word_idx ON vocab (word, lang);
